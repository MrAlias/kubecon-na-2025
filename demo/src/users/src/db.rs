use libsql::Builder;

pub struct Database {
    db: libsql::Database,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create a remote database connection via libsql
        let db = Builder::new_remote(url.to_string(), "".to_string())
            .build()
            .await?;
        
        Ok(Database { db })
    }

    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.connect()?;
        
        // Create the users table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL
            )",
            (),
        )
        .await?;

        // Seed with initial users
        self.seed_initial_users().await?;

        Ok(())
    }

    async fn seed_initial_users(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.db.connect()?;
        
        let seed_users = vec![
            "Alice Johnson",
            "Bob Smith",
            "Carol Williams",
        ];

        for username in seed_users {
            // Insert or ignore if already exists
            let _ = conn
                .execute(
                    "INSERT OR IGNORE INTO users (username) VALUES (?1)",
                    libsql::params![username],
                )
                .await;
        }

        Ok(())
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<crate::User>, Box<dyn std::error::Error>> {
        let conn = self.db.connect()?;
        
        let mut rows = conn.query(
            "SELECT id, username FROM users WHERE id = ?1",
            libsql::params![id],
        )
        .await?;

        if let Some(row) = rows.next().await? {
            let user = crate::User {
                id: row.get(0)?,
                username: row.get(1)?,
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    pub async fn list_users(&self) -> Result<Vec<crate::User>, Box<dyn std::error::Error>> {
        let conn = self.db.connect()?;
        let mut rows = conn.query(
            "SELECT id, username FROM users ORDER BY id",
            (),
        )
        .await?;

        let mut users = Vec::new();
        while let Some(row) = rows.next().await? {
            let user = crate::User {
                id: row.get(0)?,
                username: row.get(1)?,
            };
            users.push(user);
        }

        Ok(users)
    }

    pub async fn create_or_get_user(&self, username: &str) -> Result<crate::User, Box<dyn std::error::Error>> {
        let conn = self.db.connect()?;
        
        // Try to insert the user (may fail if it already exists)
        let _ = conn.execute(
            "INSERT INTO users (username) VALUES (?1)",
            libsql::params![username],
        )
        .await;

        // Get the user (whether newly inserted or already existed)
        let mut rows = conn.query(
            "SELECT id, username FROM users WHERE username = ?1",
            libsql::params![username],
        )
        .await?;

        if let Some(row) = rows.next().await? {
            let user = crate::User {
                id: row.get(0)?,
                username: row.get(1)?,
            };
            Ok(user)
        } else {
            Err("User not found after creation".into())
        }
    }
}
