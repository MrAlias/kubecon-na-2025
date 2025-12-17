use rusqlite::{Connection, Result as SqliteResult};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }

    pub fn init(&self) -> SqliteResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL
            )",
            [],
        )?;

        // Seed with initial users
        self.seed_initial_users()?;

        Ok(())
    }

    fn seed_initial_users(&self) -> SqliteResult<()> {
        let seed_users = vec![
            "Alice Johnson",
            "Bob Smith",
            "Carol Williams",
        ];

        for username in seed_users {
            // Insert or ignore if already exists
            self.conn.execute(
                "INSERT OR IGNORE INTO users (username) VALUES (?1)",
                [username],
            )?;
        }

        Ok(())
    }

    pub fn get_user(&self, id: i32) -> SqliteResult<Option<crate::User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username FROM users WHERE id = ?1"
        )?;

        let result = stmt.query_row([id], |row| {
            Ok(crate::User {
                id: row.get(0)?,
                username: row.get(1)?,
            })
        });

        match result {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn list_users(&self) -> SqliteResult<Vec<crate::User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username FROM users ORDER BY id"
        )?;

        let users = stmt.query_map([], |row| {
            Ok(crate::User {
                id: row.get(0)?,
                username: row.get(1)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;

        Ok(users)
    }

    pub fn create_or_get_user(&self, username: &str) -> SqliteResult<crate::User> {
        // Try to insert the user
        let insert_result = self.conn.execute(
            "INSERT INTO users (username) VALUES (?1)",
            [username],
        );

        // Get the user (whether newly inserted or already existed)
        let mut stmt = self.conn.prepare(
            "SELECT id, username FROM users WHERE username = ?1"
        )?;

        let user = stmt.query_row([username], |row| {
            Ok(crate::User {
                id: row.get(0)?,
                username: row.get(1)?,
            })
        })?;

        Ok(user)
    }
}
