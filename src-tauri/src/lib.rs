// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {

    use rusqlite::{ Connection, Result };

    #[test]
    fn test() {
        assert_eq!(1,1)
    }

    #[test]
    fn test_create_connection() -> rusqlite::Result<()> {
        let conn = rusqlite::Connection::open_in_memory()?;

        conn.execute("

            CREATE TABLE IF NOT EXISTS 
            test(id INTEGER PRIMARY KEY, txt TEXT NOT NULL)",
            [])?;

        conn.execute("INSERT INTO test (txt) VALUES (?1)", ["TEST"])?;


        let res: String = conn.query_row(
            "SELECT txt from test LIMIT 1"
            , [], |row| row.get(0))?;

        assert_eq!(res, "TEST");

        Ok(())
    }
}
