use rusqlite::{params, Connection, Result};

struct DB {
    conn: Connection,
}

impl DB {
    fn connect() -> Result<Self> {
        let conn = Connection::open("Data.db")?;
        Ok(DB { conn })
    }

    fn delete_save(&mut self){
        let _delete = self.conn.execute("DROP TABLE player;", []);
    }

    fn new_save(&mut self, name: String) -> Result<()>{
        //이유는 모르겠는데 참조 안하는 변수면 앞에 언더바 주라더라 컴파일러 좋긴 좋은듯...
        //let player_init = self.conn.execute("
        //help: if this is intentional, prefix it with an underscore: `_player_init`
        
        let _player_init = self.conn.execute("
            CREATE TABLE IF NOT EXISTS player(name TEXT, strength INTEGER, sens INTEGER);
        ", []);

        let result = self.conn.execute("INSERT INTO player(name, strength, sens) VALUES(?, 1, 1);", [&name])?;
        // match result {
        //     Ok(rows_affected) => {
        //         println!("{} 행이 삽입되었습니다.", rows_affected);
        //         // 삽입 성공 처리
        //     }
        //     Err(e) => {
        //         eprintln!("삽입 실패: {}", e);
        //         // 삽입 실패 처리
        //     }
        // }

        Ok(())
    }
    
}

fn delete_save(){
    let db = DB::connect();
    let _delete = db.expect("에러!").delete_save();
}

fn new_save(name: String){
    let db = DB::connect();
    let _new_save = db.expect("에러!").new_save(name);
}


fn main() {
    // new_save("name".to_string());
    delete_save();
}
 
