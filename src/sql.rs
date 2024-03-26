use rusqlite::{params, Connection, Result};

pub fn insert_user(conn: &Connection, user_id: &str, ea_id: &str) -> Result<usize> {
    conn.execute(
        "INSERT INTO telegram_user (user_id, ea_name) VALUES (?1, ?2)",
        params![user_id, ea_id],
    )
}

pub fn query_user(conn: &Connection, user_id: &str) -> Result<String> {
    conn.query_row(
        "SELECT ea_name FROM telegram_user WHERE user_id = ?1",
        params![user_id],
        |row| row.get(0),
    )
}

pub fn delete_user(conn: &Connection, user_id: &str) -> Result<usize> {
    conn.execute(
        "DELETE FROM telegram_user WHERE user_id = ?1",
        params![user_id],
    )
}
