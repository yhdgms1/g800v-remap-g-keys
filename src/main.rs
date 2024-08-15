#![windows_subsystem = "windows"]

use windows::Win32::System::Console::GetConsoleWindow;
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use std::mem::size_of;
use std::env;
use std::path::Path;
use std::ffi::OsStr;

fn program_name() -> Option<String> {
    env::args().next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}

fn get_f_key(count: u8) -> VIRTUAL_KEY {
    match count {
        13 => VK_F13,
        14 => VK_F14,
        15 => VK_F15,
        16 => VK_F16,
        17 => VK_F17,
        18 => VK_F18,
        19 => VK_F19,
        20 => VK_F20,
        21 => VK_F21,
        22 => VK_F22,
        23 => VK_F23,
        24 => VK_F24,
        _ => unreachable!()
    }
}

fn get_key(name: String) -> VIRTUAL_KEY {
    if name.starts_with("f") {
        return get_f_key(name.clone().split_off(1).parse::<u8>().expect("Could not parse F key"));
    }

    return VK_LWIN;
}

fn main() {
    let command = program_name().expect("Could not get program name").split('.').next().to_owned().unwrap().to_string();

    // Убирает фокус
    unsafe {
        let console_window = GetConsoleWindow();
        let _ = ShowWindow(console_window, SW_HIDE);
    }

    let mut input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: get_key(command),
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    // Нажатие
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }

    input.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

    // Отжатие
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}



