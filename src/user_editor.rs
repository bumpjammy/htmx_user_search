use sqlite3::{Connection, Statement};

pub fn search_users<'a>(conn: &'a Connection, first_name: &'a str, last_name: &'a str, is_admin: bool) -> Statement<'a> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            is_admin BOOLEAN NOT NULL
        )",
    ).unwrap();
    let mut statement = "SELECT * FROM users WHERE first_name LIKE ? || '%' AND last_name LIKE ? || '%' LIMIT 1000";
    if is_admin == true {
        statement = "SELECT * FROM users WHERE first_name LIKE ? || '%' AND last_name LIKE ? || '%' AND is_admin = 1 LIMIT 1000";
    }
    let mut statement = conn.prepare(
        statement
    ).unwrap();
    statement.bind(1, first_name).unwrap();
    statement.bind(2, last_name).unwrap();
    statement
}

