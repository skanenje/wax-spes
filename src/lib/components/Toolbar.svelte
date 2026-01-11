<script lang="ts">
  import type { Tool, OpenTab } from "$lib/types";

  interface Props {
    tools: Tool[];
    openTabs: OpenTab[];
    activeTabId: string | null;
    onToolClick: (tool: Tool) => void;
    onTabClick: (sessionId: string) => void;
    onTabClose: (sessionId: string) => void;
  }

  let { tools, openTabs, activeTabId, onToolClick, onTabClick, onTabClose }: Props = $props();
</script>

<div class="toolbar">
  <div class="bookmarks">
    {#each tools as tool}
      <button class="bookmark" on:click={() => onToolClick(tool)} title={tool.name}>
        <img src={tool.icon_path} alt={tool.name} />
      </button>
    {/each}
  </div>

  <div class="tabs">
    {#each openTabs as tab}
      <div 
        class="tab" 
        class:active={activeTabId === tab.session.id}
        on:click={() => onTabClick(tab.session.id)}
      >
        <img src={tab.tool.icon_path} alt={tab.tool.name} />
        <span>{tab.tool.name}</span>
        <button 
          class="close" 
          on:click|stopPropagation={() => onTabClose(tab.session.id)}
        >×</button>
      </div>
    {/each}
  </div>
</div>

<style>
  .toolbar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 80px;
    background: #f5f5f5;
    border-bottom: 1px solid #ddd;
    display: flex;
    flex-direction: column;
    z-index: 1000;
  }

  .bookmarks {
    height: 40px;
    display: flex;
    gap: 8px;
    padding: 4px 12px;
    background: #fff;
    border-bottom: 1px solid #e0e0e0;
    overflow-x: auto;
  }

  .bookmark {
    width: 32px;
    height: 32px;
    padding: 4px;
    border: none;
    background: transparent;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.15s;
  }

  .bookmark:hover {
    background: #f0f0f0;
  }

  .bookmark img {
    width: 24px;
    height: 24px;
  }

  .tabs {
    height: 40px;
    display: flex;
    gap: 2px;
    padding: 0 8px;
    overflow-x: auto;
    background: #e8e8e8;
  }

  .tab {
    min-width: 180px;
    max-width: 240px;
    height: 36px;
    margin-top: 4px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    gap: 8px;
    background: #d0d0d0;
    border-radius: 8px 8px 0 0;
    cursor: pointer;
    transition: background 0.15s;
  }

  .tab:hover {
    background: #c0c0c0;
  }

  .tab.active {
    background: #fff;
  }

  .tab img {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
  }

  .tab span {
    flex: 1;
    font-size: 13px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .close {
    width: 20px;
    height: 20px;
    border: none;
    background: transparent;
    border-radius: 4px;
    cursor: pointer;
    font-size: 18px;
    line-height: 1;
    color: #666;
    flex-shrink: 0;
  }

  .close:hover {
    background: rgba(0, 0, 0, 0.1);
    color: #000;
  }
</style>
