use sqlite3::Statement;

pub fn format_user_search(statement: &mut Statement) -> String{
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