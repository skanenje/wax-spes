<script lang="ts">
  import { onMount } from "svelte";
  import Toolbar from "$lib/components/Toolbar.svelte";
  import { loadTools } from "$lib/services/httpToolService";
  import { createSession } from "$lib/services/httpSessionService";
  import type { Tool, OpenTab } from "$lib/types";

  let tools: Tool[] = $state([]);
  let openTabs: OpenTab[] = $state([]);
  let activeTabId: string | null = $state(null);

  onMount(async () => {
    tools = await loadTools();

    if (window.electronAPI) {
      window.electronAPI.onTabSwitched((sessionId: string) => {
        activeTabId = sessionId;
      });

      window.electronAPI.onTabClosed((sessionId: string) => {
        openTabs = openTabs.filter(tab => tab.session.id !== sessionId);
        if (activeTabId === sessionId) {
          activeTabId = openTabs.length > 0 ? openTabs[0].session.id : null;
        }
      });
    }
  });

  async function handleToolClick(tool: Tool) {
    const session = await createSession(tool.id);
    const newTab: OpenTab = { session, tool };
    
    openTabs = [...openTabs, newTab];
    activeTabId = session.id;

    if (window.electronAPI) {
      window.electronAPI.createTab(session.id, tool.url);
    }
  }

  function handleTabClick(sessionId: string) {
    activeTabId = sessionId;
    if (window.electronAPI) {
      window.electronAPI.switchTab(sessionId);
    }
  }

  function handleTabClose(sessionId: string) {
    if (window.electronAPI) {
      window.electronAPI.closeTab(sessionId);
    }
  }
</script>

<div class="app">
  <Toolbar 
    {tools}
    {openTabs}
    {activeTabId}
    onToolClick={handleToolClick}
    onTabClick={handleTabClick}
    onTabClose={handleTabClose}
  />
</div>

<style>
  .app {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
</style>
