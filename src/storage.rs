pub mod storage {
  use chrono::{Datelike, Duration, Local};
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
      Ok(_res) => (),
      Err(_err) => (),
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
      Ok(_res) => (),
      Err(_err) => (),
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

  pub fn get_keyboard1() -> Vec<i32> {
    let today = Local::now();
    let start_of_week = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let end_of_week = start_of_week + Duration::days(7);
    let start_of_week_fmt = start_of_week.format("%Y-%m-%d").to_string();
    let end_of_week_fmt = end_of_week.format("%Y-%m-%d").to_string();
    let conn = get_conn();
    let mut stmt = conn
      .prepare(
        "SELECT COUNT(*) as count, DATE(created_at) as date FROM keyboard WHERE created_at BETWEEN ? AND ? GROUP BY DATE(created_at)",
      )
      .expect("error");

    let rows = stmt
      .query_map([start_of_week_fmt, end_of_week_fmt], |row| {
        // println!("{:?}", Ok((row.get(0)?, row.get(1))));
        Ok(row.get(0)?)
      })
      .expect("error");

    let mut data: Vec<i32> = vec![];
    for row in rows {
      data.push(row.expect("error"));
    }
    return data;
  }

  pub fn get_keyboard2() -> Vec<i32> {
    let today = Local::now().format("%Y");
    let mut date_array: [String; 12] = Default::default();
    for v in 1..=12 {
      let month = if v < 10 {
        "0".to_string() + &v.to_string()
      } else {
        (v).to_string()
      };
      date_array[v - 1] = format!("{}-{}", today, month);
    }

    let conn = get_conn();
    let mut stmt = conn
      .prepare(
        "SELECT months.month, COALESCE(data.count, 0) AS count
              FROM (
                  SELECT ? AS month
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
              ) AS months
              LEFT JOIN (
                  SELECT COUNT(*) as count, STRFTIME('%Y-%m', created_at) as month
                  FROM keyboard
                  WHERE created_at >= '2023' AND created_at < '2024'
                  GROUP BY month
              ) AS data ON months.month = data.month
              ORDER BY months.month",
      )
      .expect("error");

    let rows = stmt
      .query_map(date_array, |row| {
        // println!("{:?}", Ok((row.get(0)?, row.get(1))));
        Ok(row.get(1)?)
      })
      .expect("error");

    let mut data: Vec<i32> = vec![];
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

  pub fn get_mouse1() -> Vec<i32> {
    let today = Local::now();
    let start_of_week = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let end_of_week = start_of_week + Duration::days(7);
    let start_of_week_fmt = start_of_week.format("%Y-%m-%d").to_string();
    let end_of_week_fmt = end_of_week.format("%Y-%m-%d").to_string();
    let conn = get_conn();
    let mut stmt = conn
      .prepare(
        "SELECT COUNT(*) as count, DATE(created_at) as date FROM mouse WHERE created_at BETWEEN ? AND ? GROUP BY DATE(created_at)",
      )
      .expect("error");

    let rows = stmt
      .query_map([start_of_week_fmt, end_of_week_fmt], |row| {
        // println!("{:?}", Ok((row.get(0)?, row.get(1))));
        Ok(row.get(0)?)
      })
      .expect("error");

    let mut data: Vec<i32> = vec![];
    for row in rows {
      data.push(row.expect("error"));
    }
    return data;
  }

  pub fn get_mouse2() -> Vec<i32> {
    let today = Local::now().format("%Y");
    let mut date_array: [String; 12] = Default::default();
    for v in 1..=12 {
      let month = if v < 10 {
        "0".to_string() + &v.to_string()
      } else {
        (v).to_string()
      };
      date_array[v - 1] = format!("{}-{}", today, month);
    }

    let conn = get_conn();
    let mut stmt = conn
      .prepare(
        "SELECT months.month, COALESCE(data.count, 0) AS count
              FROM (
                  SELECT ? AS month
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
                  UNION SELECT ?
              ) AS months
              LEFT JOIN (
                  SELECT COUNT(*) as count, STRFTIME('%Y-%m', created_at) as month
                  FROM mouse
                  WHERE created_at >= '2023' AND created_at < '2024'
                  GROUP BY month
              ) AS data ON months.month = data.month
              ORDER BY months.month",
      )
      .expect("error");

    let rows = stmt
      .query_map(date_array, |row| {
        // println!("{:?}", Ok((row.get(0)?, row.get(1))));
        Ok(row.get(1)?)
      })
      .expect("error");

    let mut data: Vec<i32> = vec![];
    for row in rows {
      data.push(row.expect("error"));
    }
    return data;
  }
}
