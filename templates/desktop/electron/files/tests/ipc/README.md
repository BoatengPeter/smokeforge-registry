# IPC tests

Test the IPC boundary here: sender checks, Zod validation, result/error
normalization, and preload API shape. Mock Electron boundaries rather than
testing raw `ipcMain`.
