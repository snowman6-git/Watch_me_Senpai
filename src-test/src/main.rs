// src/main.rs

use rusqlite::Connection;

fn main() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("test.db")?;

    conn.execute("CREATE TABLE IF NOT EXISTS player_data (name TEXT, strength INTEGER, sens INTEGER);", [])?;
    conn.execute("INSERT INTO player_data (name, strength, sens) VALUES('스노우맨', '3', '4');", [])?;

    let mut stmt = conn.prepare("SELECT * FROM player_data WHERE name = ?")?;
    let mut rows = stmt.query([&"스노우맨"])?;
    if let Some(row) = rows.next()? {

        println!("Name: {}", row);

        let name: String = row.get(0)?;
        println!("Name: {}", name);
    }

    // let a = conn.execute("SELECT * FROM player_data WHERE = '스노우맨';", [])?;
    // println!("{}", a);

    // conn.execute("CREATE TABLE IF NOT EXISTS save_love (id INTEGER PRIMARY KEY, name TEXT)", [])?;
    Ok(())
}