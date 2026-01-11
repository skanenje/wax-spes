# Wax Space

A desktop app for managing your AI chat sessions in one place.

## What is Wax Space?

Wax Space gives you quick access to all your favorite AI chat tools—ChatGPT, Claude, Mistral, Gemini, and GitHub Copilot—from a single desktop application. Keep track of your conversations and switch between tools seamlessly.

## Features

- **All Your AI Tools in One Place**: Access ChatGPT, Claude, Mistral, Gemini, and Copilot without juggling browser tabs
- **Session Tracking**: Keep a history of your AI conversations organized by tool
- **Privacy First**: All your data stays on your computer—nothing is sent to external servers
- **Fast & Native**: Built with Electron for smooth performance on Windows, macOS, and Linux

## Getting Started

### Download & Install

Download the latest version for your operating system from the [Releases](../../releases) page.

### Running from Source

If you want to run the app from source code:

1. Make sure you have [Node.js](https://nodejs.org/) and [Rust](https://www.rust-lang.org/tools/install) installed
2. Clone this repository and navigate to the folder
3. Install dependencies:
   ```bash
   npm install
   ```
4. Run the app in development mode (requires 2 terminals):
   
   Terminal 1 - Start the frontend:
   ```bash
   npm run dev
   ```
   
   Terminal 2 - Start the app:
   ```bash
   npm start
   ```

To build a production version:
```bash
npm run dist
```

Or build for a specific platform:
```bash
npm run dist:linux   # Linux (AppImage, deb)
npm run dist:mac     # macOS (dmg, zip)
npm run dist:win     # Windows (installer, portable)
```

The compiled app will be in the `dist/` folder.

### Using the A

1. Launch Wax Space
2. Select an AI tool from the available options
3. Start a new session or continue from your recent sessions
4. Your session history is automatically saved locally

## Your Data

All session data is stored locally on your computer in a SQLite database. No information is sent to external servers. You can find your data at:

- **Windows**: `%APPDATA%/com.zedolph.wax-space/cognitive-shell.db`
- **macOS**: `~/Library/Application Support/com.zedolph.wax-space/cognitive-shell.db`
- **Linux**: `~/.local/share/com.zedolph.wax-space/cognitive-shell.db`

## Building Releases

To create distributable packages:

1. Install dependencies:
   ```bash
   npm install
   ```

2. Build for your platform:
   ```bash
   npm run dist          # Current platform
   npm run dist:linux    # Linux (AppImage, deb)
   npm run dist:mac      # macOS (dmg, zip) 
   npm run dist:win      # Windows (installer, portable)
   ```

3. Find the installers in the `dist/` folder

Note: Cross-platform builds may require additional setup. Building for macOS requires a Mac, and Windows builds work best on Windows.

## Supported AI Tools

- ChatGPT (OpenAI)
- Claude (Anthropic)
- Mistral AI
- Gemini (Google)
- GitHub Copilot
- Grok

## License

MIT
