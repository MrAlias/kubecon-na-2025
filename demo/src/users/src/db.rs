use postgres::Client;

// All database functions are synchronous (not async) to maintain proper trace
// context propagation when using OpenTelemetry eBPF Instrumentation (OBI).
// Async database operations can lose trace context when spawning tasks.

pub fn init_database(client: &mut Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create the users table
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username TEXT UNIQUE NOT NULL
            )",
            &[],
        )?;

    // Seed with initial users
    seed_initial_users(client)?;

    Ok(())
}

fn seed_initial_users(client: &mut Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let seed_users = vec![
        "Alice Johnson",
        "Bob Smith",
        "Carol Williams",
    ];

    for username in seed_users {
        // Insert or ignore if already exists
        let _ = client
            .execute(
                "INSERT INTO users (username) VALUES ($1) ON CONFLICT (username) DO NOTHING",
                &[&username],
            );
    }

    Ok(())
}

pub fn get_user(client: &mut Client, id: i32) -> Result<Option<crate::User>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = client
        .query(
            "SELECT id, username FROM users WHERE id = $1",
            &[&id],
        )?;

    if let Some(row) = rows.first() {
        let user = crate::User {
            id: row.get(0),
            username: row.get(1),
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub fn list_users(client: &mut Client) -> Result<Vec<crate::User>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = client
        .query("SELECT id, username FROM users ORDER BY id", &[])?;

    let users = rows
        .iter()
        .map(|row| crate::User {
            id: row.get(0),
            username: row.get(1),
        })
        .collect();

    Ok(users)
}

pub fn create_or_get_user(client: &mut Client, username: &str) -> Result<crate::User, Box<dyn std::error::Error + Send + Sync>> {
    // Try to insert the user (ignores if it already exists)
    let _ = client
        .execute(
            "INSERT INTO users (username) VALUES ($1) ON CONFLICT (username) DO NOTHING",
            &[&username],
        );

    // Get the user (whether newly inserted or already existed)
    let rows = client
        .query(
            "SELECT id, username FROM users WHERE username = $1",
            &[&username],
        )?;

    if let Some(row) = rows.first() {
        let user = crate::User {
            id: row.get(0),
            username: row.get(1),
        };
        Ok(user)
    } else {
        Err("User not found after creation".into())
    }
}
