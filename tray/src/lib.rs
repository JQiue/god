use std::env::temp_dir;
use std::fs::File;
use std::io::Write;

pub fn run() {
  println!("tray");
  let mut app = systray::Application::new().expect("Can't create window!");
  // w.set_tooltip(&"Whatever".to_string());
  let temp_dir = temp_dir();
  let ico_path: std::path::PathBuf = temp_dir.join("trayicon.ico");
  let ico_data = include_bytes!("./favicon.ico");
  let mut file = File::create(&ico_path).unwrap();
  file.write_all(ico_data).expect("Failed to write to file");
  drop(file);

  app
    .set_icon_from_file(ico_path.to_str().expect("error"))
    .expect("error");

  app
    .add_menu_item("Print a thing", |_| {
      println!("Printing a thing!");
      Ok::<_, systray::Error>(())
    })
    .expect("add menu item error");

  app
    .add_menu_item("Add Menu Item", |window| {
      window.add_menu_item("Interior item", |_| {
        println!("what");
        Ok::<_, systray::Error>(())
      })?;
      window.add_menu_separator()?;
      Ok::<_, systray::Error>(())
    })
    .expect("add menu item error");
  app.add_menu_separator().expect("add menu separator error");

  app
    .add_menu_item("退出", |window| {
      window.quit();
      Ok::<_, systray::Error>(())
    })
    .expect("add menu item error");

  let _ = app.wait_for_message();
}
