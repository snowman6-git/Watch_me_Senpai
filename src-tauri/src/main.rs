// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{process};

fn main() {
    // dating_with_stupids_lib::run()
    tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![link])
    .invoke_handler(tauri::generate_handler![close, new_save, delete_save, list_save])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

#[tauri::command]
fn close() {
    println!("프로그램을 종료합니다.");
    process::exit(0); // 0은 성공적인 종료를 의미합니다. 다른 값을 사용하여 오류를 나타낼 수도 있습니다.
}

use rusqlite::{Connection, Result};

struct DB {
    conn: Connection,
}

impl DB {
    fn connect() -> Result<Self> {
        let conn = Connection::open("Data.db")?;
        Ok(DB { conn })
    }

    //나중에 에러처리 하기 , rusqlite::Error
    fn list_save(&mut self) -> Result<i64> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM player")?;
        let result = stmt.query_row([], |row| {
            let value: i64 = row.get(0)?;
            Ok(value)
        })?;
        Ok(result)
    }

    fn delete_save(&mut self){
        let _delete = self.conn.execute("DROP TABLE player;", []);
    }

    fn new_save(&mut self, name: String) -> bool{
        let _player_init = self.conn.execute("
            CREATE TABLE IF NOT EXISTS player(name TEXT NOT NULL, strength INTEGER DEFAULT 1, sens INTEGER DEFAULT 1, is_biru INTEGER DEFAULT 0);
        ", []);
        
        match self.conn.execute(
            if name == "비루" || name == "biru" {
                "INSERT INTO player(name, strength, sens, is_biru) VALUES(?, 1, 1, 1);"
            } else {
                "INSERT INTO player(name, strength, sens, is_biru) VALUES(?, 1, 1, 0);"
            },
            [&name]
        ) {
            Ok(_) => {
                println!("플레이어 정보가 성공적으로 삽입되었습니다.");
                true
            }
            Err(e) => {
                println!("오류 발생: {:?}", e);
                false
            }
        }
    }

}

#[tauri::command]
fn list_save() -> Option<i64> {
    let db = DB::connect();
    let list_save = db.expect("에러!").list_save();
    
    match list_save {
        Ok(value) => Some(value),
        Err(e) => {
            println!("에러 발생: {:?}", e);
            Some(0)
        }
    }
}

#[tauri::command]
fn delete_save(){
    let db = DB::connect();
    let _delete = db.expect("에러!").delete_save();
}

#[tauri::command]
fn new_save(name: String) -> bool{
    let db = DB::connect();
    let save_is = db.expect("에러!").new_save(name);
    if save_is == true {
        true
    } else {
        false
    }
}