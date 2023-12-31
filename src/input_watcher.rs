pub mod input_watcher {

  use inputbot::{
    KeybdKey::{self},
    MouseButton::{self},
  };

  use crate::storage::storage::{insert_keyboard, insert_mouse};

  pub fn run() {
    KeybdKey::bind_all(|e| insert_keyboard(convert_keybd_key(e)));
    MouseButton::bind_all(|e| insert_mouse(convert_mouse_button(e)));
    inputbot::handle_input_events();
  }

  fn convert_keybd_key(k: KeybdKey) -> String {
    match k {
      KeybdKey::BackspaceKey => return "Backspace".to_string(),
      KeybdKey::TabKey => "Tab".to_string(),
      KeybdKey::EnterKey => "Enter".to_string(),
      KeybdKey::EscapeKey => "Escape".to_string(),
      KeybdKey::SpaceKey => "Space".to_string(),
      KeybdKey::PageUpKey => "PageUp".to_string(),
      KeybdKey::PageDownKey => "PageDown".to_string(),
      KeybdKey::EndKey => "End".to_string(),
      KeybdKey::HomeKey => "Home".to_string(),
      KeybdKey::LeftKey => "Left".to_string(),
      KeybdKey::UpKey => "Up".to_string(),
      KeybdKey::RightKey => "Right".to_string(),
      KeybdKey::DownKey => "Down".to_string(),
      KeybdKey::InsertKey => "Insert".to_string(),
      KeybdKey::DeleteKey => "Delete".to_string(),
      KeybdKey::Numrow0Key => "Numrow0".to_string(),
      KeybdKey::Numrow1Key => "Numrow1".to_string(),
      KeybdKey::Numrow2Key => "Numrow2".to_string(),
      KeybdKey::Numrow3Key => "Numrow3".to_string(),
      KeybdKey::Numrow4Key => "Numrow4".to_string(),
      KeybdKey::Numrow5Key => "Numrow5".to_string(),
      KeybdKey::Numrow6Key => "Numrow6".to_string(),
      KeybdKey::Numrow7Key => "Numrow7".to_string(),
      KeybdKey::Numrow8Key => "Numrow8".to_string(),
      KeybdKey::Numrow9Key => "Numrow9".to_string(),
      KeybdKey::AKey => "A".to_string(),
      KeybdKey::BKey => "B".to_string(),
      KeybdKey::CKey => "C".to_string(),
      KeybdKey::DKey => "D".to_string(),
      KeybdKey::EKey => "E".to_string(),
      KeybdKey::FKey => "F".to_string(),
      KeybdKey::GKey => "G".to_string(),
      KeybdKey::HKey => "H".to_string(),
      KeybdKey::IKey => "I".to_string(),
      KeybdKey::JKey => "J".to_string(),
      KeybdKey::KKey => "K".to_string(),
      KeybdKey::LKey => "L".to_string(),
      KeybdKey::MKey => "M".to_string(),
      KeybdKey::NKey => "N".to_string(),
      KeybdKey::OKey => "O".to_string(),
      KeybdKey::PKey => "P".to_string(),
      KeybdKey::QKey => "Q".to_string(),
      KeybdKey::RKey => "R".to_string(),
      KeybdKey::SKey => "S".to_string(),
      KeybdKey::TKey => "T".to_string(),
      KeybdKey::UKey => "U".to_string(),
      KeybdKey::VKey => "V".to_string(),
      KeybdKey::WKey => "W".to_string(),
      KeybdKey::XKey => "X".to_string(),
      KeybdKey::YKey => "Y".to_string(),
      KeybdKey::ZKey => "Z".to_string(),
      KeybdKey::LSuper => "WinLeft".to_string(),
      KeybdKey::RSuper => "WinRight".to_string(),
      KeybdKey::Numpad0Key => "Numpad0".to_string(),
      KeybdKey::Numpad1Key => "Numpad1".to_string(),
      KeybdKey::Numpad2Key => "Numpad2".to_string(),
      KeybdKey::Numpad3Key => "Numpad3".to_string(),
      KeybdKey::Numpad4Key => "Numpad4".to_string(),
      KeybdKey::Numpad5Key => "Numpad5".to_string(),
      KeybdKey::Numpad6Key => "Numpad6".to_string(),
      KeybdKey::Numpad7Key => "Numpad7".to_string(),
      KeybdKey::Numpad8Key => "Numpad8".to_string(),
      KeybdKey::Numpad9Key => "Numpad9".to_string(),
      KeybdKey::F1Key => "F1".to_string(),
      KeybdKey::F2Key => "F2".to_string(),
      KeybdKey::F3Key => "F3".to_string(),
      KeybdKey::F4Key => "F4".to_string(),
      KeybdKey::F5Key => "F5".to_string(),
      KeybdKey::F6Key => "F6".to_string(),
      KeybdKey::F7Key => "F7".to_string(),
      KeybdKey::F8Key => "F8".to_string(),
      KeybdKey::F9Key => "F9".to_string(),
      KeybdKey::F10Key => "F10".to_string(),
      KeybdKey::F11Key => "F11".to_string(),
      KeybdKey::F12Key => "F12".to_string(),
      KeybdKey::F13Key => "".to_string(),
      KeybdKey::F14Key => "".to_string(),
      KeybdKey::F15Key => "".to_string(),
      KeybdKey::F16Key => "".to_string(),
      KeybdKey::F17Key => "".to_string(),
      KeybdKey::F18Key => "".to_string(),
      KeybdKey::F19Key => "".to_string(),
      KeybdKey::F20Key => "".to_string(),
      KeybdKey::F21Key => "".to_string(),
      KeybdKey::F22Key => "".to_string(),
      KeybdKey::F23Key => "".to_string(),
      KeybdKey::F24Key => "".to_string(),
      KeybdKey::NumLockKey => "NumLock".to_string(),
      KeybdKey::ScrollLockKey => "ScrollLock".to_string(),
      KeybdKey::CapsLockKey => "CapsLock".to_string(),
      KeybdKey::LShiftKey => "ShiftLeft".to_string(),
      KeybdKey::RShiftKey => "ShiftRight".to_string(),
      KeybdKey::LControlKey => "CtrlLeft".to_string(),
      KeybdKey::RControlKey => "CtrlRight".to_string(),
      KeybdKey::LAltKey => "AltLeft".to_string(),
      KeybdKey::RAltKey => "AltRight".to_string(),
      KeybdKey::BrowserBackKey => "".to_string(),
      KeybdKey::BrowserForwardKey => "".to_string(),
      KeybdKey::BrowserRefreshKey => "".to_string(),
      KeybdKey::VolumeMuteKey => "".to_string(),
      KeybdKey::VolumeDownKey => "".to_string(),
      KeybdKey::VolumeUpKey => "".to_string(),
      KeybdKey::MediaNextTrackKey => "".to_string(),
      KeybdKey::MediaPrevTrackKey => "".to_string(),
      KeybdKey::MediaStopKey => "".to_string(),
      KeybdKey::MediaPlayPauseKey => "".to_string(),
      KeybdKey::BackquoteKey => "Backquote".to_string(),
      KeybdKey::SlashKey => "Slash".to_string(),
      KeybdKey::BackslashKey => "Backslash".to_string(),
      KeybdKey::CommaKey => "Comma".to_string(),
      KeybdKey::PeriodKey => "Period".to_string(),
      KeybdKey::MinusKey => "Minus".to_string(),
      KeybdKey::QuoteKey => "Quote".to_string(),
      KeybdKey::SemicolonKey => "Semicolon".to_string(),
      KeybdKey::LBracketKey => "BracketLeft".to_string(),
      KeybdKey::RBracketKey => "BracketRight".to_string(),
      KeybdKey::EqualKey => "Equal".to_string(),
      KeybdKey::OtherKey(_) => "Other".to_string(),
    }
  }

  fn convert_mouse_button(m: MouseButton) -> String {
    match m {
      MouseButton::LeftButton => return "Left".to_string(),
      MouseButton::MiddleButton => "Middle".to_string(),
      MouseButton::RightButton => "Right".to_string(),
      MouseButton::X1Button => "X1".to_string(),
      MouseButton::X2Button => "X2".to_string(),
      MouseButton::OtherButton(_) => "Other".to_string(),
    }
  }
}
