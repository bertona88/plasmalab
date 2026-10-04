const DATABASE = 'plasmalab-experiments';
const STORE = 'checkpoints';
const KEY = 'current';
const MAX_BYTES = 8 * 1024 * 1024;

function openDatabase(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DATABASE, 1);
    request.onupgradeneeded = () => request.result.createObjectStore(STORE);
    request.onsuccess = () => resolve(request.result);
    request.onerror = () =>
      reject(
        new Error(
          'Browser storage is unavailable. Export a checkpoint file instead.',
        ),
      );
    request.onblocked = () =>
      reject(new Error('Close other PlasmaLab tabs and try again.'));
  });
}

async function transact<T>(
  mode: IDBTransactionMode,
  action: (store: IDBObjectStore) => IDBRequest<T>,
): Promise<T> {
  const db = await openDatabase();
  return new Promise((resolve, reject) => {
    const transaction = db.transaction(STORE, mode);
    const request = action(transaction.objectStore(STORE));
    transaction.oncomplete = () => {
      db.close();
      resolve(request.result);
    };
    transaction.onerror = transaction.onabort = () => {
      db.close();
      reject(
        new Error(
          'Could not access the saved checkpoint. Your running experiment is unchanged; try exporting a file.',
        ),
      );
    };
  });
}

export async function saveLocal(snapshot: string): Promise<void> {
  if (new Blob([snapshot]).size > MAX_BYTES)
    throw new Error('Checkpoint exceeds the 8 MB save limit.');
  await transact('readwrite', (store) => store.put(snapshot, KEY));
}

export async function loadLocal(): Promise<string | null> {
  const saved: unknown = await transact('readonly', (store) => store.get(KEY));
  if (saved === undefined) return null;
  if (typeof saved !== 'string')
    throw new Error('Saved checkpoint is invalid. Import a backup file.');
  return saved;
}

export function exportSnapshot(snapshot: string): void {
  const url = URL.createObjectURL(
    new Blob([snapshot], { type: 'application/json' }),
  );
  const link = document.createElement('a');
  link.href = url;
  link.download = `plasmalab-${new Date().toISOString().replace(/[:.]/g, '-')}.json`;
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

export async function readSnapshotFile(file: File): Promise<string> {
  if (file.size > MAX_BYTES)
    throw new Error('Checkpoint exceeds the 8 MB import limit.');
  const text = await file.text();
  // Validation and atomic state replacement belong to the Rust model.
  try {
    JSON.parse(text);
  } catch {
    throw new Error(
      'This file is not valid JSON. Choose a PlasmaLab checkpoint.',
    );
  }
  return text;
}
