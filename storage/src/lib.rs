use chrono::Local;
use dirs::home_dir;
use rusqlite::Connection;

fn get_conn() -> Connection {
  let home = home_dir().unwrap();
  let database_path = home.join("Documents").join("God").join("database.db");
  // let database_path = "./test.db";
  let conn = Connection::open(database_path).unwrap();
  return conn;
}

pub fn create_table() {
  let conn = get_conn();
  conn
    .execute_batch(
      "
         create table if not exists keyboard (
             id integer primary key AUTOINCREMENT,
             name,
             created_at
         );
         create table if not exists mouse (
             id integer primary key AUTOINCREMENT,
             name,
             created_at
         );",
    )
    .expect("err");
}

pub fn insert_keyboard(name: String) {
  let conn = get_conn();
  match conn.execute(
    "insert into keyboard (name, created_at) values(?,?)",
    [name, Local::now().format("%Y-%m-%d %H:%M:%S").to_string()],
  ) {
    Ok(res) => println!("{res}"),
    Err(err) => println!("{:?}", err),
  }
}

pub fn insert_mouse(name: String) {
  let conn = get_conn();
  match conn.execute(
    "insert into mouse (name,created_at) values(?,?)",
    [name, Local::now().format("%Y-%m-%d %H:%M:%S").to_string()],
  ) {
    Ok(res) => println!("{res}"),
    Err(err) => println!("{:?}", err),
  }
}
