use chrono::Local;
use dirs::home_dir;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Keyboard {
  pub name: String,
  pub created_at: String,
}

#[derive(Debug)]
pub struct Mouse {
  pub name: String,
  pub created_at: String,
}

fn get_conn() -> Connection {
  let home = home_dir().unwrap();
  let database_path = home.join("Documents").join("God").join("database.db");
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

  let keyboard = Keyboard {
    name,
    created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
  };

  match conn.execute(
    "insert into keyboard (name, created_at) values(?,?)",
    [keyboard.name, keyboard.created_at],
  ) {
    Ok(res) => println!("{res}"),
    Err(err) => println!("{:?}", err),
  }
}

pub fn insert_mouse(name: String) {
  let conn = get_conn();
  let mouse = Mouse {
    name,
    created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
  };

  match conn.execute(
    "insert into mouse (name,created_at) values(?,?)",
    [mouse.name, mouse.created_at],
  ) {
    Ok(res) => println!("{res}"),
    Err(err) => println!("{:?}", err),
  }
}

pub fn get_keyboard() -> Vec<(String, i32)> {
  let conn = get_conn();
  let mut stmt = conn
    .prepare("SELECT name, COUNT(name) as count FROM keyboard GROUP BY name ORDER BY count DESC")
    .expect("error");
  let rows = stmt
    .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
    .expect("error");
  let mut data: Vec<(String, i32)> = vec![];
  for row in rows {
    data.push(row.expect("error"));
  }
  return data;
}

pub fn get_mouse() -> Vec<(String, i32)> {
  let conn = get_conn();
  let mut stmt = conn
    .prepare("SELECT name, COUNT(name) as count FROM mouse GROUP BY name ORDER BY count DESC")
    .expect("error");
  let rows = stmt
    .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
    .expect("error");
  let mut data: Vec<(String, i32)> = vec![];
  for row in rows {
    data.push(row.expect("error"));
  }
  return data;
}
