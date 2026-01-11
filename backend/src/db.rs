use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon_path: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolSession {
    pub id: String,
    pub tool_id: String,
    pub title: Option<String>,
    pub last_active_at: i64,
    pub created_at: i64,
    pub group_id: Option<String>,
    pub position: i32,
    pub pinned: bool,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS tools (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                icon_path TEXT,
                enabled BOOLEAN DEFAULT TRUE
            );

            CREATE TABLE IF NOT EXISTS tool_sessions (
                id TEXT PRIMARY KEY,
                tool_id TEXT NOT NULL,
                title TEXT,
                last_active_at INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                group_id TEXT,
                position INTEGER DEFAULT 0,
                pinned BOOLEAN DEFAULT FALSE,
                FOREIGN KEY (tool_id) REFERENCES tools(id)
            );
            "#,
        )?;
        
        // Migration: Add missing columns if they don't exist
        self.migrate_add_columns()?;
        
        self.seed_tools()?;
        Ok(())
    }

    fn migrate_add_columns(&self) -> Result<()> {
        // Check if group_id column exists, if not add it
        let table_info: Vec<String> = self.conn
            .prepare("PRAGMA table_info(tool_sessions)")?
            .query_map([], |row| row.get(1))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        if !table_info.contains(&"group_id".to_string()) {
            println!("[LOG] Adding group_id column to tool_sessions");
            self.conn.execute("ALTER TABLE tool_sessions ADD COLUMN group_id TEXT", [])?;
        }

        if !table_info.contains(&"position".to_string()) {
            println!("[LOG] Adding position column to tool_sessions");
            self.conn.execute("ALTER TABLE tool_sessions ADD COLUMN position INTEGER DEFAULT 0", [])?;
        }

        if !table_info.contains(&"pinned".to_string()) {
            println!("[LOG] Adding pinned column to tool_sessions");
            self.conn.execute("ALTER TABLE tool_sessions ADD COLUMN pinned BOOLEAN DEFAULT FALSE", [])?;
        }

        Ok(())
    }

    fn seed_tools(&self) -> Result<()> {
        let tools = vec![
            ("chatgpt", "ChatGPT", "https://chatgpt.com", "/openai-color.svg"),
            ("claude", "Claude", "https://claude.ai", "/claude-color.svg"),
            ("mistral", "Mistral", "https://mistral.ai", "/mistral-color.svg"),
            ("gemini", "Gemini", "https://gemini.google.com", "/gemini-color.svg"),
            ("copilot", "Copilot", "https://copilot.microsoft.com", "/copilot-color.svg"),
            ("deepseek", "DeepSeek", "https://chat.deepseek.com", "/deepseek-color.svg"),
            ("perplexity", "Perplexity", "https://perplexity.ai", "/perplexity-color.svg"),
            ("grok", "Grok", "https://grok.com", "/grok.svg"),
            ("notebooklm", "NotebookLM", "https://notebooklm.google", "/notebooklm.svg"),
            ("v0", "v0", "https://v0.dev", "/v0.svg"),
            ("sora", "Sora", "https://sora.com", "/sora-color.svg"),
            ("azureai", "Azure AI Foundry", "https://ai.azure.com", "/azureai-color.svg"),
            ("vertexai", "Vertex AI", "https://console.cloud.google.com", "/vertexai-color.svg"),
            ("openrouter", "OpenRouter", "https://openrouter.ai", "/openrouter.svg"),
            ("huggingface", "Hugging Face", "https://huggingface.co/chat", "/huggingface-color.svg"),
        ];

        for (id, name, url, icon) in tools {
            self.conn.execute(
                "INSERT OR REPLACE INTO tools (id, name, url, icon_path, enabled) VALUES (?1, ?2, ?3, ?4, TRUE)",
                rusqlite::params![id, name, url, icon],
            )?;
        }

        Ok(())
    }

    pub fn get_all_tools(&self) -> Result<Vec<Tool>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, url, icon_path, enabled FROM tools WHERE enabled = TRUE ORDER BY name"
        )?;
        
        let tools = stmt.query_map([], |row| {
            Ok(Tool {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                icon_path: row.get(3)?,
                enabled: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(tools)
    }

    pub fn create_session(&self, tool_id: String) -> Result<ToolSession> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        
        self.conn.execute(
            "INSERT INTO tool_sessions (id, tool_id, title, last_active_at, created_at, position, pinned) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![id, tool_id, None::<String>, now, now, 0, false],
        )?;
        
        Ok(ToolSession {
            id,
            tool_id,
            title: None,
            last_active_at: now,
            created_at: now,
            group_id: None,
            position: 0,
            pinned: false,
        })
    }

    pub fn get_all_sessions(&self) -> Result<Vec<ToolSession>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, tool_id, title, last_active_at, created_at, group_id, position, pinned FROM tool_sessions ORDER BY pinned DESC, position ASC, last_active_at DESC"
        )?;
        
        let sessions = stmt.query_map([], |row| {
            Ok(ToolSession {
                id: row.get(0)?,
                tool_id: row.get(1)?,
                title: row.get(2)?,
                last_active_at: row.get(3)?,
                created_at: row.get(4)?,
                group_id: row.get(5)?,
                position: row.get(6)?,
                pinned: row.get(7)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(sessions)
    }

    pub fn update_session_activity(&self, session_id: String) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        self.conn.execute(
            "UPDATE tool_sessions SET last_active_at = ?1 WHERE id = ?2",
            rusqlite::params![now, session_id],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_tab_pinned(&self, session_id: String, pinned: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE tool_sessions SET pinned = ?1 WHERE id = ?2",
            rusqlite::params![pinned, session_id],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_tab_group(&self, session_id: String, group_id: Option<String>) -> Result<()> {
        self.conn.execute(
            "UPDATE tool_sessions SET group_id = ?1 WHERE id = ?2",
            rusqlite::params![group_id, session_id],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update_tab_position(&self, session_id: String, position: i32) -> Result<()> {
        self.conn.execute(
            "UPDATE tool_sessions SET position = ?1 WHERE id = ?2",
            rusqlite::params![position, session_id],
        )?;
        Ok(())
    }
}