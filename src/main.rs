mod user_editor;
mod html_formatter;

#[macro_use] extern crate rocket;

use std::path::PathBuf;
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
    let mut statement = user_editor::search_users(&conn, first_name, last_name, is_admin);
    let mut result = html_formatter::format_user_search(&mut statement);
    result
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![files, index, search])
}