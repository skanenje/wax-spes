import type { Tool } from "../types/index";

const API_BASE = "http://localhost:3001/api";

export async function loadTools(): Promise<Tool[]> {
  try {
    console.log("[LOG] loadTools called");
    const response = await fetch(`${API_BASE}/tools`);
    const tools = await response.json();
    console.log("[LOG] loadTools success:", tools);
    return tools;
  } catch (error) {
    console.error("[ERROR] Failed to load tools:", error);
    return [];
  }
}