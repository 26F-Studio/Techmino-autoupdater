use std::path::{Path, PathBuf};
use std::env::consts::OS;
use std::process::Command;

use crate::lang;

const SRC_MAC: &str = "/Users/user/Library/Application Support/LOVE/Techmino/updateExtract/";
const SRC_LNX: &str = "~/.local/share/love/Techmino/updateExtract";

pub fn get_src() -> PathBuf {
    match OS {
        "windows" => { 
            let appdata = std::env::var_os("APPDATA").expect(lang::get_entry("appdata_not_found"));
            return Path::new(&appdata).join("Techmino").join("updateExtract");
        }
        "macos"   => { return Path::new(SRC_MAC).to_path_buf(); }
        "linux"   => { return Path::new(SRC_LNX).to_path_buf(); }
        _ => panic!("{}", lang::get_entry("unsupported_os"))
    }
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
