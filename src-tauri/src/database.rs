use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub priority: String,
    pub category: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("todos.db")?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                completed BOOLEAN NOT NULL DEFAULT 0,
                priority TEXT NOT NULL DEFAULT 'medium',
                category TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                due_date TEXT
            )",
            [],
        )?;
        
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
    
    pub fn get_all_todos(&self) -> Result<Vec<Todo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, description, completed, priority, category, created_at, updated_at, due_date 
             FROM todos ORDER BY created_at DESC"
        )?;
        
        let todos = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                completed: row.get(3)?,
                priority: row.get(4)?,
                category: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                due_date: row.get(8)?,
            })
        })?.collect::<Result<Vec<_>>>()?;
        
        Ok(todos)
    }
    
    pub fn create_todo(&self, todo: CreateTodo) -> Result<Todo> {
        let conn = self.conn.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc().to_string();
        let priority = todo.priority.unwrap_or_else(|| "medium".to_string());
        
        conn.execute(
            "INSERT INTO todos (id, title, description, completed, priority, created_at, updated_at, due_date)
             VALUES (?1, ?2, ?3, 0, ?4, ?5, ?6, ?7)",
            params![id, todo.title, todo.description, priority, now, now, todo.due_date],
        )?;
        
        Ok(Todo {
            id,
            title: todo.title,
            description: todo.description,
            completed: false,
            priority,
            category: None,
            created_at: now.clone(),
            updated_at: now,
            due_date: todo.due_date,
        })
    }
    
    pub fn update_todo(&self, id: &str, updates: UpdateTodo) -> Result<Todo> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().naive_utc().to_string();
        
        // 获取当前待办
        let mut stmt = conn.prepare(
            "SELECT id, title, description, completed, priority, category, created_at, updated_at, due_date 
             FROM todos WHERE id = ?1"
        )?;
        
        let todo = stmt.query_row(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                completed: row.get(3)?,
                priority: row.get(4)?,
                category: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                due_date: row.get(8)?,
            })
        })?;
        
        // 更新字段
        let title = updates.title.unwrap_or(todo.title);
        let description = updates.description.or(todo.description);
        let completed = updates.completed.unwrap_or(todo.completed);
        let priority = updates.priority.unwrap_or(todo.priority);
        let due_date = updates.due_date.or(todo.due_date);
        
        conn.execute(
            "UPDATE todos SET title = ?1, description = ?2, completed = ?3, priority = ?4, 
             updated_at = ?5, due_date = ?6 WHERE id = ?7",
            params![title, description, completed, priority, now, due_date, id],
        )?;
        
        Ok(Todo {
            id: id.to_string(),
            title,
            description,
            completed,
            priority,
            category: todo.category,
            created_at: todo.created_at,
            updated_at: now,
            due_date,
        })
    }
    
    pub fn delete_todo(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
        Ok(())
    }
    
    pub fn toggle_todo(&self, id: &str) -> Result<Todo> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().naive_utc().to_string();
        
        conn.execute(
            "UPDATE todos SET completed = NOT completed, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        
        let mut stmt = conn.prepare(
            "SELECT id, title, description, completed, priority, category, created_at, updated_at, due_date 
             FROM todos WHERE id = ?1"
        )?;
        
        let todo = stmt.query_row(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                completed: row.get(3)?,
                priority: row.get(4)?,
                category: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                due_date: row.get(8)?,
            })
        })?;
        
        Ok(todo)
    }
    
    pub fn clear_completed(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM todos WHERE completed = 1", [])?;
        Ok(())
    }
}