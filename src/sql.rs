use rusqlite::{params, Connection, Result};

pub fn init_db(conn: &Connection) -> Result<usize> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            user_id TEXT NOT NULL,
            ea_id TEXT NOT NULL
        )",
        [],
    )
}
pub fn insert_user(conn: &Connection, user_id: &str, ea_id: &str) -> Result<usize> {
    conn.execute(
        "INSERT INTO users (user_id, ea_id) VALUES (?1, ?2)",
        params![user_id, ea_id],
    )
}

pub fn query_user(conn: &Connection, user_id: &str) -> Result<String> {
    conn.query_row(
        "SELECT ea_id FROM users WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    )
}

pub fn delete_user(conn: &Connection, user_id: &str) -> Result<usize> {
    conn.execute("DELETE FROM users WHERE user_id = ?1", params![user_id])
}
