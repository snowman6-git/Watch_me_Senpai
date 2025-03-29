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
    fn new_save(&mut self){
        //이유는 모르겠는데 참조 안하는 변수면 앞에 언더바 주라더라 컴파일러 좋긴 좋은듯...
        //let player_init = self.conn.execute("
        //help: if this is intentional, prefix it with an underscore: `_player_init`
        let _player_init = self.conn.execute("
            CREATE TABLE IF NOT EXISTS player(name TEXT, strength INTEGER, sens INTEGER)
        ", []);
        // INSERT INTO player(strength, sens) VALUES('1', '1');

        
        // println!("OK!");
        // self.conn.execute("INSERT INTO player(name) VALUES('스노우맨');", []);
    }

    // fn init_save(&self){
    //     self.conn.execute("", params)
    // }
    
    // fn query(&self, sql: &str) -> Result<(), rusqlite::Error> {
    //     self.conn.execute(sql, []);   
    //     Ok(())
    // }
    // fn save(&self, sql: &str) -> Result<(), rusqlite::Error> {
    //     self.conn.execute(sql, []);
    //     Ok(())
    // }
}

fn delete_save(){
    let db = DB::connect();
    let _delete = db.expect("에러!").delete_save();
}

fn main() {
    // delete_save();
    let db = DB::connect();
    let _new_save = db.expect("에러!").new_save();
}
 
