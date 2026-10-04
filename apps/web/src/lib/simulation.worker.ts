import initialize, { Lab } from '../wasm/plasmalab_core';
import type { Command, Response } from './contracts';

let lab: Lab | undefined;

function dispatch(command: Command): Response {
  if (!lab) throw new Error('The simulation is not ready.');
  const response = JSON.parse(
    lab.dispatch(JSON.stringify(command)),
  ) as Response;
  if (!response.ok)
    throw new Error(response.error ?? 'Model rejected the command.');
  return response;
}

// A single serial queue also protects initialization and future asynchronous executors.
let queue = Promise.resolve();
self.onmessage = ({
  data,
}: MessageEvent<{ id: number; kind: string; value?: unknown }>) => {
  queue = queue.then(async () => {
    try {
      let result: unknown;
      switch (data.kind) {
        case 'init':
          await initialize();
          lab = new Lab('tearing');
          result = dispatch({ type: 'catalog' }).catalog;
          break;
        case 'command':
          result = dispatch(data.value as Command).frame;
          break;
        case 'snapshot':
          result = JSON.stringify(dispatch({ type: 'snapshot' }).snapshot);
          break;
        case 'restore':
          if (
            typeof data.value !== 'string' ||
            data.value.length > 8 * 1024 * 1024
          )
            throw new Error('Checkpoint is invalid or exceeds the 8 MB limit.');
          result = dispatch({
            type: 'restore',
            snapshot: JSON.parse(data.value),
          }).frame;
          break;
        default:
          throw new Error('Unknown execution request.');
      }
      self.postMessage({ id: data.id, result });
    } catch (error) {
      self.postMessage({
        id: data.id,
        error: error instanceof Error ? error.message : String(error),
      });
    }
  });
};
