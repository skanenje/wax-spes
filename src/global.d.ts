/// <reference types="@sveltejs/kit" />

interface Window {
  electronAPI?: {
    platform: string;
    createTab: (sessionId: string, toolUrl: string) => void;
    switchTab: (sessionId: string) => void;
    closeTab: (sessionId: string) => void;
    onTabCreated: (callback: (sessionId: string) => void) => void;
    onTabSwitched: (callback: (sessionId: string) => void) => void;
    onTabClosed: (callback: (sessionId: string) => void) => void;
  };
}
