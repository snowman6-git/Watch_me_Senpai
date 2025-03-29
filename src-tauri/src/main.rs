// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process;

fn main() {
    // dating_with_stupids_lib::run()
    tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![link])
    .invoke_handler(tauri::generate_handler![close])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

#[tauri::command]
fn close() {
    println!("프로그램을 종료합니다.");
    process::exit(0); // 0은 성공적인 종료를 의미합니다. 다른 값을 사용하여 오류를 나타낼 수도 있습니다.
}