const { app, BrowserWindow, BrowserView, ipcMain } = require('electron');
const { spawn } = require('child_process');
const path = require('path');

let mainWindow = null;
let backendProcess = null;
const tabs = new Map(); // sessionId -> { view, tool, session }
let activeTabId = null;

const TOOLBAR_HEIGHT = 80;
const isDev = !app.isPackaged;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.cjs')
    }
  });

  // Load from dev server in development, from built files in production
  if (isDev) {
    mainWindow.loadURL('http://localhost:1420');
  } else {
    const indexPath = path.join(__dirname, 'build', 'index.html');
    console.log('[ELECTRON] Loading from:', indexPath);
    console.log('[ELECTRON] __dirname:', __dirname);
    mainWindow.loadFile(indexPath).catch(err => {
      console.error('[ELECTRON] Failed to load:', err);
    });
  }
  
  // Open DevTools in development
  if (isDev) {
    mainWindow.webContents.openDevTools();
  }
  
  mainWindow.on('resize', () => {
    if (activeTabId && tabs.has(activeTabId)) {
      updateActiveViewBounds();
    }
  });
}

function updateActiveViewBounds() {
  if (!activeTabId || !tabs.has(activeTabId)) return;
  
  const bounds = mainWindow.getContentBounds();
  const view = tabs.get(activeTabId).view;
  view.setBounds({ 
    x: 0, 
    y: TOOLBAR_HEIGHT, 
    width: bounds.width, 
    height: bounds.height - TOOLBAR_HEIGHT 
  });
  view.setAutoResize({ width: true, height: true });
}

function createTab(sessionId, toolUrl) {
  if (tabs.has(sessionId)) {
    switchToTab(sessionId);
    return;
  }

  const view = new BrowserView({
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      partition: `persist:session-${sessionId}`
    }
  });

  view.webContents.loadURL(toolUrl);
  tabs.set(sessionId, { view });

  if (!activeTabId) {
    activeTabId = sessionId;
    mainWindow.setBrowserView(view);
    // Delay to ensure window is ready
    setTimeout(() => updateActiveViewBounds(), 100);
  }

  mainWindow.webContents.send('tab-created', sessionId);
}

function switchToTab(sessionId) {
  if (!tabs.has(sessionId)) return;
  
  activeTabId = sessionId;
  const { view } = tabs.get(sessionId);
  mainWindow.setBrowserView(view);
  updateActiveViewBounds();
  
  mainWindow.webContents.send('tab-switched', sessionId);
}

function closeTab(sessionId) {
  if (!tabs.has(sessionId)) return;
  
  const { view } = tabs.get(sessionId);
  
  if (activeTabId === sessionId) {
    mainWindow.removeBrowserView(view);
    
    const tabIds = Array.from(tabs.keys());
    const currentIndex = tabIds.indexOf(sessionId);
    tabs.delete(sessionId);
    
    const remainingIds = Array.from(tabs.keys());
    if (remainingIds.length > 0) {
      const nextId = remainingIds[Math.min(currentIndex, remainingIds.length - 1)];
      switchToTab(nextId);
    } else {
      activeTabId = null;
    }
  } else {
    tabs.delete(sessionId);
  }
  
  view.webContents.destroy();
  mainWindow.webContents.send('tab-closed', sessionId);
}

ipcMain.on('create-tab', (event, { sessionId, toolUrl }) => {
  createTab(sessionId, toolUrl);
});

ipcMain.on('switch-tab', (event, sessionId) => {
  switchToTab(sessionId);
});

ipcMain.on('close-tab', (event, sessionId) => {
  closeTab(sessionId);
});

function startBackend() {
  const fs = require('fs');
  const backendPath = isDev 
    ? './backend/target/release/wax-space-server'
    : path.join(__dirname, 'backend/target/release/wax-space-server');
  
  console.log('[ELECTRON] Starting backend from:', backendPath);
  console.log('[ELECTRON] isDev:', isDev);
  console.log('[ELECTRON] __dirname:', __dirname);
  
  // Ensure backend file is executable
  try {
    const stats = fs.statSync(backendPath);
    if (!stats.isFile()) {
      console.error('[ELECTRON] Backend path is not a file:', backendPath);
      return;
    }
    
    // Make sure it's executable
    const isExecutable = (stats.mode & 0o111) !== 0;
    if (!isExecutable) {
      console.log('[ELECTRON] Making backend executable...');
      fs.chmodSync(backendPath, 0o755);
    }
  } catch (err) {
    console.error('[ELECTRON] Failed to prepare backend:', err.message);
    return;
  }
  
  backendProcess = spawn(backendPath, [], {
    cwd: isDev ? path.dirname(backendPath) : __dirname,
    stdio: 'inherit'
  });
  
  backendProcess.on('error', (err) => {
    console.error('[ELECTRON] Backend spawn error:', err);
  });
}

app.whenReady().then(() => {
  startBackend();
  setTimeout(() => {
    createWindow();
  }, 2000);

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', () => {
  if (backendProcess) {
    backendProcess.kill();
  }
  if (process.platform !== 'darwin') app.quit();
});

app.on('before-quit', () => {
  if (backendProcess) {
    backendProcess.kill();
  }
});
