use std::path::PathBuf;
use rusqlite::{Connection, Result, Error as SqliteError};
use serde::{Deserialize, Serialize};

const DB_PATH: &str = "~/.teammate/teammate.db";

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub content: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub priority: String,
    pub status: String,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub issue: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug)]
pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let db_path = shellexpand::tilde(DB_PATH).to_string();
        
        // Ensure directory exists
        if let Some(parent) = PathBuf::from(&db_path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(1), Some(e.to_string())))?;
        }
        
        let conn = Connection::open(&db_path)?;
        
        // Initialize database schema
        Self::init_schema(&conn)?;
        
        Ok(Storage { conn })
    }
    
    fn init_schema(conn: &Connection) -> Result<()> {
        // Create todos table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                file TEXT,
                line INTEGER,
                priority TEXT DEFAULT 'medium',
                status TEXT DEFAULT 'open',
                author TEXT,
                issue TEXT,
                created_at INTEGER,
                updated_at INTEGER
            )",
            [],
        )?;
        
        // Create tags table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL
            )",
            [],
        )?;
        
        // Create todo_tags junction table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todo_tags (
                todo_id TEXT,
                tag_id INTEGER,
                PRIMARY KEY (todo_id, tag_id),
                FOREIGN KEY (todo_id) REFERENCES todos(id),
                FOREIGN KEY (tag_id) REFERENCES tags(id)
            )",
            [],
        )?;
        
        // Create scan_sessions table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS scan_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL,
                started_at INTEGER,
                completed_at INTEGER,
                files_scanned INTEGER,
                todos_found INTEGER
            )",
            [],
        )?;
        
        Ok(())
    }
    
    // CRUD operations for Todo
    pub fn create_todo(&self, todo: &Todo) -> Result<()> {
        self.conn.execute(
            "INSERT INTO todos (id, content, file, line, priority, status, author, issue, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![
                todo.id,
                todo.content,
                todo.file,
                todo.line.map(|l| l as i64),
                todo.priority,
                todo.status,
                todo.author,
                todo.issue,
                todo.created_at as i64,
                todo.updated_at as i64
            ],
        )?;
        
        // Insert tags
        for tag in &todo.tags {
            self.add_tag(tag)?;
            let tag_id = self.get_tag_id(tag)?;
            if let Some(tag_id) = tag_id {
                self.conn.execute(
                    "INSERT OR IGNORE INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
                    rusqlite::params![todo.id, tag_id],
                )?;
            }
        }
        
        Ok(())
    }
    
    pub fn get_todo(&self, id: &str) -> Result<Option<Todo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, file, line, priority, status, author, issue, created_at, updated_at
             FROM todos WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            let tags = self.get_todo_tags(id).unwrap_or_default();
            let todo = Todo {
                id: row.get(0)?,
                content: row.get(1)?,
                file: row.get(2)?,
                line: row.get::<_, Option<i64>>(3)?.map(|l| l as usize),
                priority: row.get(4)?,
                status: row.get(5)?,
                tags,
                author: row.get(6)?,
                issue: row.get(7)?,
                created_at: row.get::<_, i64>(8)? as u64,
                updated_at: row.get::<_, i64>(9)? as u64,
            };
            Ok(Some(todo))
        } else {
            Ok(None)
        }
    }
    
    pub fn list_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, file, line, priority, status, author, issue, created_at, updated_at
             FROM todos ORDER BY created_at DESC"
        )?;
        
        let mut todos = Vec::new();
        let rows = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                content: row.get(1)?,
                file: row.get(2)?,
                line: row.get::<_, Option<i64>>(3)?.map(|l| l as usize),
                priority: row.get(4)?,
                status: row.get(5)?,
                tags: Vec::new(), // Placeholder
                author: row.get(6)?,
                issue: row.get(7)?,
                created_at: row.get::<_, i64>(8)? as u64,
                updated_at: row.get::<_, i64>(9)? as u64,
            })
        })?;
        
        for row in rows {
            let mut todo = row?;
            // Fetch tags separately
            todo.tags = self.get_todo_tags(&todo.id).unwrap_or_default();
            todos.push(todo);
        }
        
        Ok(todos)
    }
    
    pub fn update_todo(&self, todo: &Todo) -> Result<()> {
        self.conn.execute(
            "UPDATE todos SET content = ?1, file = ?2, line = ?3, priority = ?4, status = ?5,
             author = ?6, issue = ?7, updated_at = ?8 WHERE id = ?9",
            rusqlite::params![
                todo.content,
                todo.file,
                todo.line.map(|l| l as i64),
                todo.priority,
                todo.status,
                todo.author,
                todo.issue,
                todo.updated_at as i64,
                todo.id
            ],
        )?;
        
        // Update tags
        let todo_id = todo.id.clone();
        self.conn.execute("DELETE FROM todo_tags WHERE todo_id = ?1", [&todo_id])?;
        for tag in &todo.tags {
            self.add_tag(tag)?;
            let tag_id = self.get_tag_id(tag)?;
            if let Some(tag_id) = tag_id {
                self.conn.execute(
                    "INSERT OR IGNORE INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
                    rusqlite::params![todo_id, tag_id],
                )?;
            }
        }
        
        Ok(())
    }
    
    pub fn delete_todo(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM todo_tags WHERE todo_id = ?1", [id])?;
        self.conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;
        Ok(())
    }
    
    // Tag operations
    fn add_tag(&self, name: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
            [name],
        )?;
        Ok(())
    }
    
    fn get_tag_id(&self, name: &str) -> Result<Option<i64>> {
        let mut stmt = self.conn.prepare("SELECT id FROM tags WHERE name = ?1")?;
        let mut rows = stmt.query([name])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
    
    fn get_todo_tags(&self, todo_id: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.name FROM tags t
             JOIN todo_tags tt ON t.id = tt.tag_id
             WHERE tt.todo_id = ?1"
        )?;
        
        let mut tags = Vec::new();
        let rows = stmt.query_map([todo_id], |row| Ok(row.get(0)?))?;
        for row in rows {
            tags.push(row?);
        }
        
        Ok(tags)
    }
    
    pub fn get_all_tags(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM tags ORDER BY name")?;
        let mut tags = Vec::new();
        let rows = stmt.query_map([], |row| Ok(row.get(0)?))?;
        for row in rows {
            tags.push(row?);
        }
        Ok(tags)
    }
    
    // Statistics
    pub fn get_stats(&self) -> Result<TodoStats> {
        let total: i64 = self.conn.query_row("SELECT COUNT(*) FROM todos", [], |r| r.get(0))?;
        let open: i64 = self.conn.query_row("SELECT COUNT(*) FROM todos WHERE status = 'open'", [], |r| r.get(0))?;
        let in_progress: i64 = self.conn.query_row("SELECT COUNT(*) FROM todos WHERE status = 'in_progress'", [], |r| r.get(0))?;
        let resolved: i64 = self.conn.query_row("SELECT COUNT(*) FROM todos WHERE status = 'resolved'", [], |r| r.get(0))?;
        
        Ok(TodoStats {
            total: total as usize,
            open: open as usize,
            in_progress: in_progress as usize,
            resolved: resolved as usize,
        })
    }
}

#[derive(Debug)]
pub struct TodoStats {
    pub total: usize,
    pub open: usize,
    pub in_progress: usize,
    pub resolved: usize,
}
