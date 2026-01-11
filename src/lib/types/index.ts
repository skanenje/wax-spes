export interface Tool {
  id: string;
  name: string;
  url: string;
  icon_path: string;
  enabled: boolean;
}

export interface ToolSession {
  id: string;
  tool_id: string;
  title: string | null;
  last_active_at: number;
  created_at: number;
  group_id: string | null;
  position: number;
  pinned: boolean;
}

export interface OpenTab {
  session: ToolSession;
  tool: Tool;
}
