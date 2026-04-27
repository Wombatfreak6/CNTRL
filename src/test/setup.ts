import '@testing-library/jest-dom';

// Mock Tauri IPC
beforeAll(() => {
  Object.defineProperty(window, '__TAURI_INTERNALS__', {
    value: {
      invoke: (cmd: string, args: any) => {
        if (cmd === 'get_tabs') return Promise.resolve([]);
        return Promise.resolve();
      }
    }
  });
});
