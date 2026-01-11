# Wax Space - Project Structure

This document outlines the modular architecture of the wax-space project.

## Frontend Structure (`src/`)

### Directory Layout

```
src/
├── lib/
│   ├── components/       # Reusable Svelte components
│   ├── services/         # Business logic and API calls
│   ├── types/           # TypeScript type definitions
│   ├── utils/           # Utility functions
│   └── stores/          # Svelte stores (for future state management)
├── routes/
│   ├── +page.svelte     # Main page component
│   ├── +page.svelte.css # Page styles
│   └── +layout.ts       # Layout component
└── app.html             # HTML entry point
```

### Components (`src/lib/components/`)

- **LandingView.svelte**: Initial landing page showing available tools
  - Displays tool grid
  - Shows history button
  - Handles tool selection

- **BrowserView.svelte**: Main application view with open tabs
  - Manages tab bar
  - Displays active tool
  - Handles navigation between tabs

- **Tab.svelte**: Individual tab component
  - Shows tool icon and name
  - Close button functionality
  - Active state styling

- **HistorySidebar.svelte**: Session history sidebar
  - Lists previous sessions
  - Allows resuming sessions
  - Shows activity timestamps

### Services (`src/lib/services/`)

- **toolService.ts**: Tool-related API calls
  - `loadTools()` - Fetch available tools
  - `getToolById()` - Find tool by ID
  - `openToolInBrowser()` - Open URL in system browser

- **sessionService.ts**: Session management
  - `loadSessions()` - Fetch all sessions
  - `createSession()` - Create new session
  - `updateSessionActivity()` - Update last active timestamp
  - `toggleTabPin()` - Pin/unpin tabs
  - `setTabGroup()` - Assign tabs to groups
  - `updateTabPosition()` - Reorder tabs

- **tabService.ts**: Tab lifecycle management
  - `openTool()` - Open a tool in a new tab
  - `switchTab()` - Switch between open tabs
  - `closeTab()` - Close a tab
  - `resumeSession()` - Resume a previous session

### Types (`src/lib/types/`)

- **index.ts**: Central type definitions
  - `Tool` - AI tool definition
  - `ToolSession` - User session record
  - `OpenTab` - Active tab representation

### Utils (`src/lib/utils/`)

- **formatting.ts**: Text formatting utilities
  - `formatDate()` - Format timestamps to relative time

- **tabUtils.ts**: Tab manipulation utilities
  - `groupTabsByTool()` - Group tabs by tool ID
  - `sortTabsByPin()` - Sort tabs by pin status
  - `getRecentTabs()` - Get recently used tabs
  - `findTabByName()` - Search tabs by name

### Stores (`src/lib/stores/`)

Reserved for future Svelte stores implementation for global state management.

---

## Backend Structure (`src-tauri/src/`)

### File Organization

```
src-tauri/src/
├── main.rs          # Application entry point and setup
├── commands.rs      # Tauri command handlers (exported)
├── state.rs         # Application state structure
├── db.rs            # Database layer
└── webview_manager.rs # Webview management
```

### Modules

- **main.rs**: Application initialization
  - Sets up Tauri app
  - Initializes database
  - Registers command handlers
  - Configures application state

- **state.rs**: Centralized application state
  - `AppState` struct containing database connection

- **commands.rs**: Tauri IPC command definitions
  - All frontend-callable commands
  - Error handling and logging
  - State management delegation to DB

- **db.rs**: Database layer (existing)
  - SQLite operations
  - Schema management
  - CRUD operations for tools and sessions

---

## Communication Flow

### Frontend → Backend
1. Component calls service function
2. Service invokes Tauri command
3. Command handler processes request
4. Database executes operation
5. Result returned to service
6. Component state updated

### Example: Opening a Tool
```
LandingView → handleOpenTool() → tabService.openTool() 
  → invoke("create_session") → commands::create_session() 
  → db.create_session() → Returns ToolSession
```

---

## Best Practices

### Component Development
- Keep components focused on UI rendering
- Pass callbacks as props for actions
- Use TypeScript for type safety

### Service Layer
- Group related API calls
- Handle logging and error management
- Provide clean interfaces for components

### Type Safety
- Define all data structures in `types/index.ts`
- Use strict TypeScript settings
- Avoid `any` types

### Rust Backend
- Commands are thin wrappers around DB operations
- Centralized state management in `AppState`
- Consistent error handling and logging

---

## Future Improvements

1. **State Management**: Implement Svelte stores for global state
2. **Error Handling**: Create error boundary components
3. **Testing**: Add unit tests for services
4. **API Layer**: Consider creating dedicated API client class
5. **Caching**: Implement response caching in services
6. **Analytics**: Add event tracking
