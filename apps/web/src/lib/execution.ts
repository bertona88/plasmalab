import type { Catalog, Command, Frame } from './contracts';

export interface Execution {
  readonly catalog: Catalog;
  command(command: Command): Promise<Frame>;
  snapshot(): Promise<string>;
  restore(snapshot: string): Promise<Frame>;
  dispose(): void;
}

type Reply = { id: number; result?: unknown; error?: string };

/** Only commands, observations and disposable presentation samples cross here.
 * The worker owns the model; a later executor need not expose its internal arrays.
 */
export async function createExecution(): Promise<Execution> {
  const worker = new Worker(
    new URL('./simulation.worker.ts', import.meta.url),
    { type: 'module' },
  );
  let nextId = 0;
  let closed = false;
  const pending = new Map<
    number,
    { resolve: (value: unknown) => void; reject: (reason: Error) => void }
  >();

  function fail(message: string) {
    closed = true;
    for (const { reject } of pending.values()) reject(new Error(message));
    pending.clear();
    worker.terminate();
  }

  worker.onmessage = ({ data }: MessageEvent<Reply>) => {
    const request = pending.get(data.id);
    if (!request) return;
    pending.delete(data.id);
    if (data.error) request.reject(new Error(data.error));
    else request.resolve(data.result);
  };
  worker.onerror = () =>
    fail(
      'The simulation worker could not start or stopped unexpectedly. Reload PlasmaLab to restart.',
    );
  worker.onmessageerror = () =>
    fail(
      'The simulation returned an unreadable response. Reload PlasmaLab to restart.',
    );

  function request<T>(kind: string, value?: unknown): Promise<T> {
    if (closed)
      return Promise.reject(new Error('Simulation worker is closed.'));
    const id = ++nextId;
    return new Promise<T>((resolve, reject) => {
      pending.set(id, { resolve: (value) => resolve(value as T), reject });
      worker.postMessage({ id, kind, value });
    });
  }

  try {
    const catalog = await request<Catalog>('init');
    return {
      catalog,
      command: (command) => request<Frame>('command', command),
      snapshot: () => request<string>('snapshot'),
      restore: (snapshot) => request<Frame>('restore', snapshot),
      dispose: () => fail('Simulation closed.'),
    };
  } catch (error) {
    fail('Simulation initialization failed.');
    throw error;
  }
}
