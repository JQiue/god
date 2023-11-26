pub mod tray {
  use std::env::temp_dir;
  use std::fs::File;
  use std::io::Write;
  use std::process::exit;

  use crate::activity::activity::preview;

  pub fn run() {
    let mut app = systray::Application::new().expect("Can't create window!");
    app
      .set_tooltip("God watches you")
      .expect("Failed to set tooltip");
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
      .add_menu_item("预览", |_| {
        preview();
        Ok::<_, systray::Error>(())
      })
      .expect("add menu item error");

    app.add_menu_separator().expect("add menu separator error");

    app
      .add_menu_item("退出", |_| {
        exit(0);
        Ok::<_, systray::Error>(())
      })
      .expect("add menu item error");

    let _ = app.wait_for_message();
  }
}
