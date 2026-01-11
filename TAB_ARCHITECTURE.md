# Tab Management Architecture

## Overview

Wax Space now uses Electron's native BrowserView API to implement browser-like tab management with full isolation and smooth switching.

## Key Components

### 1. Electron Main Process (electron-main.cjs)

**Responsibilities:**
- Manages BrowserView instances for each tab
- Handles tab lifecycle (create, switch, close)
- Provides session isolation via partition strategy
- Controls view bounds and positioning

**Tab Storage:**
```javascript
tabs = Map<sessionId, { view: BrowserView }>
```

**Key Functions:**
- `createTab(sessionId, toolUrl)` - Creates new BrowserView with isolated partition
- `switchToTab(sessionId)` - Switches active BrowserView
- `closeTab(sessionId)` - Destroys BrowserView and cleans up
- `updateActiveViewBounds()` - Adjusts view size on window resize

### 2. Preload Script (preload.cjs)

**Responsibilities:**
- Exposes secure IPC bridge to renderer
- Provides tab control APIs
- Handles bidirectional communication

**Exposed APIs:**
```typescript
electronAPI: {
  createTab(sessionId, toolUrl)
  switchTab(sessionId)
  closeTab(sessionId)
  onTabCreated(callback)
  onTabSwitched(callback)
  onTabClosed(callback)
}
```

### 3. Frontend (SvelteKit)

**Main Page (+page.svelte):**
- Manages tab state (openTabs, activeTabId)
- Coordinates with Electron via IPC
- Handles user interactions

**Toolbar Component:**
- Bookmark bar for quick tool access
- Tab strip for open sessions
- Tab switching and closing UI

## Data Flow

### Opening a Tool

```
User clicks bookmark
  → Frontend creates session (API call)
  → Frontend adds to openTabs state
  → Frontend calls electronAPI.createTab()
  → Main process creates BrowserView
  → Main process loads tool URL in view
  → Main process sends 'tab-created' event
```

### Switching Tabs

```
User clicks tab
  → Frontend updates activeTabId
  → Frontend calls electronAPI.switchTab()
  → Main process calls setBrowserView()
  → Main process updates bounds
  → Main process sends 'tab-switched' event
```

### Closing Tabs

```
User clicks close button
  → Frontend calls electronAPI.closeTab()
  → Main process destroys BrowserView
  → Main process removes from tabs Map
  → Main process switches to next tab if needed
  → Main process sends 'tab-closed' event
  → Frontend updates openTabs state
```

## Session Isolation

Each tab uses a unique partition:
```javascript
partition: `persist:session-${sessionId}`
```

This provides:
- Separate cookie storage
- Isolated localStorage/sessionStorage
- Independent cache
- Separate service workers

## Layout

```
┌─────────────────────────────────────┐
│ Bookmark Bar (40px)                 │ ← Quick tool access
├─────────────────────────────────────┤
│ Tab Strip (40px)                    │ ← Open tabs
├─────────────────────────────────────┤
│                                     │
│                                     │
│ Active BrowserView                  │ ← Tool content
│                                     │
│                                     │
└─────────────────────────────────────┘
```

## Benefits

1. **Native Performance**: BrowserView is faster than webview/iframe
2. **Full Isolation**: Each tab has complete storage isolation
3. **DevTools Access**: Each tab has its own DevTools
4. **Smooth Switching**: Instant tab switching without reload
5. **Memory Efficient**: Views are destroyed when tabs close
6. **Security**: Proper context isolation and IPC boundaries

## Future Enhancements

- Tab reordering (drag & drop)
- Tab pinning
- Tab groups
- Session persistence across app restarts
- Keyboard shortcuts (Cmd/Ctrl+T, Cmd/Ctrl+W, etc.)
