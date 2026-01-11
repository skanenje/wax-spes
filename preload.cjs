const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
  platform: process.platform,
  createTab: (sessionId, toolUrl) => {
    ipcRenderer.send('create-tab', { sessionId, toolUrl });
  },
  switchTab: (sessionId) => {
    ipcRenderer.send('switch-tab', sessionId);
  },
  closeTab: (sessionId) => {
    ipcRenderer.send('close-tab', sessionId);
  },
  onTabCreated: (callback) => {
    ipcRenderer.on('tab-created', (event, sessionId) => callback(sessionId));
  },
  onTabSwitched: (callback) => {
    ipcRenderer.on('tab-switched', (event, sessionId) => callback(sessionId));
  },
  onTabClosed: (callback) => {
    ipcRenderer.on('tab-closed', (event, sessionId) => callback(sessionId));
  }
});
