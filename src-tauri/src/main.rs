// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{process, result};

fn main() {
    // dating_with_stupids_lib::run()
    tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![link])
    .invoke_handler(tauri::generate_handler![close, new_save, delete_save, name_check])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

#[tauri::command]
fn close() {
    println!("프로그램을 종료합니다.");
    process::exit(0); // 0은 성공적인 종료를 의미합니다. 다른 값을 사용하여 오류를 나타낼 수도 있습니다.
}

use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};
use serde_json::json;


struct DB {
    conn: Connection,
}
struct SavePreset {
    id: i8,
    name: String,
}

impl DB {
    fn connect() -> Result<Self> {
        let conn = Connection::open("Data.db")?;
        Ok(DB { conn })
    }

    //나중에 에러처리 하기 , rusqlite::Error
    fn list_save(&mut self) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM player")?;
        let _save_list = stmt.query_map([], |row| {
            let save_list_result = SavePreset {
                id: row.get(0)?,
                name: row.get(1)?,
            };
            println!("{}", save_list_result.id);
            Ok(save_list_result)
        })?;
        Ok(())
    }

    fn delete_save(&mut self){
        let _delete = self.conn.execute("DROP TABLE player;", []);
    }

    fn new_save(&mut self, name: &str) -> Option<i16> {
        let _player_init = self.conn.execute("
            CREATE TABLE IF NOT EXISTS player(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, strength INTEGER DEFAULT 1, sens INTEGER DEFAULT 1, is_biru INTEGER DEFAULT 0);
        ", []);

        let result = match name {
            "비루" => {
                let result = self.conn.execute("INSERT INTO player(name, strength, sens, is_biru) VALUES(?, 1, 1, 1);", [&name]);
                match result {
                    Ok(_) => Some(200),
                    Err(e) => {
                        println!("{}", e);
                        Some(400)
                    }
                }
            }
            _ => {
                let result = self.conn.execute("INSERT INTO player(name, strength, sens, is_biru) VALUES(?, 1, 1, 0);", [&name]);
                match result {
                    Ok(_) => Some(200),
                    Err(e) => {
                        println!("{}", e);
                        Some(400)
                    }
                }
            }
        };
        result
     }

}

#[tauri::command]
fn name_check(name: &str) -> Option<i64>{
    let name = name.trim(); //공백제거
    match name {
        "스노우맨" => Some(403),
        "최일한" => Some(406),
        _ => Some(200)
    }
}

// #[tauri::command]
// fn list_save() ->Result<(), rusqlite::Error>{
    // let db = DB::connect();
    // let list_save = db.expect("에러!").list_save();
    // Some(list_sa ve)

    // match list_save {
    //     Ok(value) => {
    //         Some(value)
    //     },
    //     Err(e) => {
    //         println!("에러 발생: {:?}", e);
    //         Some(400)
    //     },
    //     _ => {
    //         println!("!!")
        
    //     }
    // }
// }

#[tauri::command]
fn delete_save(){
    let db = DB::connect();
    let _delete = db.expect("에러!").delete_save();
}

#[tauri::command]
fn new_save(name: String) -> Option<i16>{
    let db = DB::connect();
    let save_is = db.expect("에러!").new_save(&name);
    save_is
}