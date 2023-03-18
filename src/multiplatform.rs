use std::path::{Path, PathBuf};
use std::env::consts::OS;
use std::process::Command;
use std::fs;

use crate::lang;

const DIR_MAC: &str = "/Users/user/Library/Application Support/LOVE";
const DIR_LNX: &str = "~/.local/share/love";

pub fn get_src() -> PathBuf {
    match OS {
        "windows" => { 
            let appdata = std::env::var_os("APPDATA").expect(lang::get_entry("appdata_not_found"));
            return Path::new(&appdata).join(r"Techmino\updateExtract");
        }
        "macos"   => { return Path::new(DIR_MAC).join("Techmino/updateExtract").to_path_buf(); }
        "linux"   => { return Path::new(DIR_LNX).join("Techmino/updateExtract").to_path_buf(); }
        _ => panic!("{}", lang::get_entry("unsupported_os"))
    }
}

pub fn get_lang() -> String {
    let path = match OS {
        "windows" => {
            let appdata = std::env::var_os("APPDATA").expect("APPDATA environment variable not set!");
            Path::new(&appdata).join(r"Techmino\conf\settings")
        },
        "macos" => { Path::new(DIR_MAC).join("Techmino/conf/settings").to_path_buf() },
        "linux" => { Path::new(DIR_LNX).join("Techmino/conf/settings").to_path_buf() }
        _ => panic!("This program only supports Windows, macOS, and Linux.")
    };
    let config = fs::read_to_string(path).unwrap_or_default();
    let lang = match config.find("\"locale\":\"") {
        Some(start) => {
            let end = match config[start + 10..].find("\"") {
                Some(pos) => pos,
                None => config.len() - start - 10,
            };
            config[start + 10..start + 10 + end].to_owned()
        }
        None => "en".to_owned(),
    };
    return lang;
}

pub fn start_techmino() {
    match OS {
        "windows" => { 
            Command::new("Techmino.exe").spawn().expect(lang::get_entry("launch_fail"));
        }
        "macos" => {
            Command::new("open").arg("./Techmino.app").spawn().expect(lang::get_entry("launch_fail"));
        }
        _ => {
            Command::new("./Techmino").spawn().expect(lang::get_entry("launch_fail"));
        }
    }
}
