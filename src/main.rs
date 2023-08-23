#[macro_use] extern crate rocket;

use std::path::PathBuf;
use rocket::Config;
use rocket::fs::NamedFile;
use sqlite3::Connection;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("webpages/index.html").await.ok()
}

#[get("/<file..>")]
async fn files(mut file: PathBuf) -> Option<NamedFile> {
    file = PathBuf::from("webpages").join(file);
    let file_path: String = file.to_str()?.to_string();
    NamedFile::open(file_path).await.ok()
}

#[post("/search_name", data = "<name>")]
async fn search(name: &str) -> String {
    if name.len() < 10 {
        return "".to_string();
    }
    let name = name.split("&").collect::<Vec<&str>>();
    let first_name = name[0].split("=").collect::<Vec<&str>>()[1];
    let last_name = name[1].split("=").collect::<Vec<&str>>()[1];
    let is_admin = name.len() > 2;
    let conn = Connection::open("db/users.db").unwrap();
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
    let mut result = String::new();
    while let sqlite3::State::Row = statement.next().unwrap() {
        let first_name: String = statement.read::<String>(1).unwrap();
        let last_name: String = statement.read::<String>(2).unwrap();
        let is_admin: bool = statement.read::<i64>(3).unwrap() != 0;
        result.push_str(&format!("{} {} - ", first_name, last_name));
        if is_admin {
            result.push_str("Admin<br>");
        } else {
            result.push_str("User<br>");
        }
    }
    if result.is_empty() {
        result = "No results found".to_string();
    }
    result
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![files, index, search])
}