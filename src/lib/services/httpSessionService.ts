import type { ToolSession } from "../types/index";

const API_BASE = "http://localhost:3001/api";

export async function loadSessions(): Promise<ToolSession[]> {
  try {
    console.log("[LOG] loadSessions called");
    const response = await fetch(`${API_BASE}/sessions`);
    const sessions = await response.json();
    console.log("[LOG] loadSessions success:", sessions);
    return sessions;
  } catch (error) {
    console.error("[ERROR] Failed to load sessions:", error);
    return [];
  }
}

export async function createSession(toolId: string): Promise<ToolSession> {
  try {
    console.log("[LOG] createSession called:", toolId);
    const response = await fetch(`${API_BASE}/sessions`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ tool_id: toolId }),
    });
    const session = await response.json();
    console.log("[LOG] createSession success:", session);
    return session;
  } catch (error) {
    console.error("[ERROR] Failed to create session:", error);
    throw error;
  }
}

export async function updateSessionActivity(sessionId: string): Promise<void> {
  try {
    console.log("[LOG] updateSessionActivity called:", sessionId);
    await fetch(`${API_BASE}/sessions/${sessionId}/activity`, {
      method: "PUT",
    });
    console.log("[LOG] updateSessionActivity success");
  } catch (error) {
    console.error("[ERROR] Failed to update session activity:", error);
    throw error;
  }
}