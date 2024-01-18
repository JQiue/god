pub mod storage {
  use std::ops::Add;

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

  /** 本周按键次数 */
  pub fn get_keyboard1() -> Vec<i32> {
    let conn = get_conn();

    let today = Local::now();
    let start_of_week = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let tuesday = start_of_week + Duration::days(1);
    let wednesday = start_of_week + Duration::days(2);
    let thursday = start_of_week + Duration::days(3);
    let friday = start_of_week + Duration::days(4);
    let saturday = start_of_week + Duration::days(5);
    let end_of_week = start_of_week + Duration::days(6);
    let start_of_week_fmt = start_of_week.format("%Y-%m-%d").to_string();
    let tuesday_fmt = tuesday.format("%Y-%m-%d").to_string();
    let wednesday_fmt = wednesday.format("%Y-%m-%d").to_string();
    let thursday_fmt = thursday.format("%Y-%m-%d").to_string();
    let friday_fmt = friday.format("%Y-%m-%d").to_string();
    let saturday_fmt = saturday.format("%Y-%m-%d").to_string();
    let end_of_week_fmt = end_of_week.format("%Y-%m-%d").to_string();

    let mut stmt1 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM keyboard WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt2 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM keyboard WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt3 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM keyboard WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt4 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM keyboard WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt5 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM keyboard WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt6 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM keyboard WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt7 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM keyboard WHERE DATE(created_at) = ?",
      )
      .expect("error");

    let one_rows = stmt1
      .query_map([start_of_week_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let two_rows = stmt2
      .query_map([tuesday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let three_rows = stmt3
      .query_map([wednesday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let four_rows = stmt4
      .query_map([thursday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let five_rows = stmt5
      .query_map([friday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let six_rows = stmt6
      .query_map([saturday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let seven_rows = stmt7
      .query_map([end_of_week_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let mut data: Vec<i32> = vec![];

    for row in one_rows {
      data.push(row.expect("error"));
    }
    for row in two_rows {
      data.push(row.expect("error"));
    }
    for row in three_rows {
      data.push(row.expect("error"));
    }
    for row in four_rows {
      data.push(row.expect("error"));
    }
    for row in five_rows {
      data.push(row.expect("error"));
    }
    for row in six_rows {
      data.push(row.expect("error"));
    }
    for row in seven_rows {
      data.push(row.expect("error"));
    }
    return data;
  }

  pub fn get_keyboard2() -> Vec<i32> {
    let today = Local::now().format("%Y");
    let mut date_array: [String; 14] = Default::default();
    for v in 1..=12 {
      let month = if v < 10 {
        "0".to_string() + &v.to_string()
      } else {
        (v).to_string()
      };
      date_array[v - 1] = format!("{}-{}", today, month);
    }
    let next_year = Local::now().year().add(1).to_string();
    date_array[12] = today.to_string();
    date_array[13] = next_year;

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
                  WHERE created_at >= ? AND created_at < ?
                  GROUP BY month
              ) AS data ON months.month = data.month
              ORDER BY months.month",
      )
      .expect("error");

    let rows = stmt
      .query_map(date_array, |row| Ok(row.get(1)?))
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
    let conn = get_conn();

    let today = Local::now();
    let start_of_week = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let tuesday = start_of_week + Duration::days(1);
    let wednesday = start_of_week + Duration::days(2);
    let thursday = start_of_week + Duration::days(3);
    let friday = start_of_week + Duration::days(4);
    let saturday = start_of_week + Duration::days(5);
    let end_of_week = start_of_week + Duration::days(6);
    let start_of_week_fmt = start_of_week.format("%Y-%m-%d").to_string();
    let tuesday_fmt = tuesday.format("%Y-%m-%d").to_string();
    let wednesday_fmt = wednesday.format("%Y-%m-%d").to_string();
    let thursday_fmt = thursday.format("%Y-%m-%d").to_string();
    let friday_fmt = friday.format("%Y-%m-%d").to_string();
    let saturday_fmt = saturday.format("%Y-%m-%d").to_string();
    let end_of_week_fmt = end_of_week.format("%Y-%m-%d").to_string();

    let mut stmt1 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM mouse WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt2 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM mouse WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt3 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM mouse WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt4 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM mouse WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt5 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM mouse WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt6 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM mouse WHERE DATE(created_at) = ?",
      )
      .expect("error");
    let mut stmt7 = conn
      .prepare(
        "SELECT COUNT(name) as count, DATE(created_at) as date FROM mouse WHERE DATE(created_at) = ?",
      )
      .expect("error");

    let one_rows = stmt1
      .query_map([start_of_week_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let two_rows = stmt2
      .query_map([tuesday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let three_rows = stmt3
      .query_map([wednesday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let four_rows = stmt4
      .query_map([thursday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let five_rows = stmt5
      .query_map([friday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let six_rows = stmt6
      .query_map([saturday_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let seven_rows = stmt7
      .query_map([end_of_week_fmt], |row| Ok(row.get(0)?))
      .expect("error");
    let mut data: Vec<i32> = vec![];

    for row in one_rows {
      data.push(row.expect("error"));
    }
    for row in two_rows {
      data.push(row.expect("error"));
    }
    for row in three_rows {
      data.push(row.expect("error"));
    }
    for row in four_rows {
      data.push(row.expect("error"));
    }
    for row in five_rows {
      data.push(row.expect("error"));
    }
    for row in six_rows {
      data.push(row.expect("error"));
    }
    for row in seven_rows {
      data.push(row.expect("error"));
    }
    return data;
  }

  pub fn get_mouse2() -> Vec<i32> {
    let today = Local::now().format("%Y");
    let mut date_array: [String; 14] = Default::default();
    let next_year = Local::now().year().add(1).to_string();
    for v in 1..=12 {
      let month = if v < 10 {
        "0".to_string() + &v.to_string()
      } else {
        (v).to_string()
      };
      date_array[v - 1] = format!("{}-{}", today, month);
    }

    date_array[12] = today.to_string();
    date_array[13] = next_year;

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
                  WHERE created_at >= ? AND created_at < ?
                  GROUP BY month
              ) AS data ON months.month = data.month
              ORDER BY months.month",
      )
      .expect("error");

    let rows = stmt
      .query_map(date_array, |row| Ok(row.get(1)?))
      .expect("error");

    let mut data: Vec<i32> = vec![];
    for row in rows {
      data.push(row.expect("error"));
    }
    return data;
  }
}
