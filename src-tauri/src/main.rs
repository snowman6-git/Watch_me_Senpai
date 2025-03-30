// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{process, result};
use std::error::Error;

fn main() {
    // dating_with_stupids_lib::run()
    tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![link])
    .invoke_handler(tauri::generate_handler![close, text_input])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

#[tauri::command]
fn close() {
    println!("프로그램을 종료합니다.");
    process::exit(0); // 0은 성공적인 종료를 의미합니다. 다른 값을 사용하여 오류를 나타낼 수도 있습니다.
}

#[tauri::command]
fn text_input(text: String) {
    println!("{}", text)
}

// use rusqlite::{params, Connection, Result};
// struct DB {
//     conn: Connection,
// }

// impl DB {
//     fn connect() -> Result<Self> {
//         let conn = Connection::open("Data.db")?;
//         Ok(DB { conn })
//     }

//     fn list_save(&mut self) -> Result<bool> {


//         let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM player")?;
//         let mut result = stmt.query_row([], |row| row.get(0))?;
    
//         println!("{}", result);

        
//         Ok(true)
//         // Ok(false)

//     }

//     fn delete_save(&mut self){
//         let _delete = self.conn.execute("DROP TABLE player;", []);
//     }

    

//     fn new_save(&mut self, name: String) -> Result<()>{
//         let _player_init = self.conn.execute("
//             CREATE TABLE IF NOT EXISTS player(name TEXT, strength INTEGER DEFAULT 1, sens INTEGER DEFAULT 1);
//         ", []);
//         let result = self.conn.execute("INSERT INTO player(name, strength, sens) VALUES(?, 1, 1);", [&name])?;
//         Ok(())
//     }
// }

// #[tauri::command]
// fn delete_save(){
//     let db = DB::connect();
//     let _delete = db.expect("에러!").delete_save();
// }

// #[tauri::command]
// fn new_save(name: String){
//     let db = DB::connect();
//     let _new_save = db.expect("에러!").new_save(name);
// }