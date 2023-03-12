use std::path::{Path, PathBuf};
use std::env::consts::OS;
use std::process::Command;

const SRC_MAC: &str = "/Users/user/Library/Application Support/LOVE/Techmino/updateExtract/";
const SRC_LNX: &str = "~/.local/share/love/Techmino/updateExtract";

pub fn get_src() -> PathBuf {
    match OS {
        "windows" => { 
            let appdata = std::env::var_os("APPDATA").expect("APPDATA env var not set");
            return Path::new(&appdata).join("Techmino").join("updateExtract");
        }
        "macos"   => { return Path::new(SRC_MAC).to_path_buf(); }
        "linux"   => { return Path::new(SRC_LNX).to_path_buf(); }
        _ => panic!("Unsupported operating system: {}", std::env::consts::OS)
    }
}


pub fn start_techmino() {
    match OS {
        "windows" => { 
            Command::new("Techmino.exe").spawn().expect("Failed to launch Techmino");
        }
        "macos" => {
            Command::new("open").arg("./Techmino.app").spawn().expect("Failed to launch Techmino");
        }
        _ => {
            Command::new("./Techmino").spawn().expect("Failed to launch Techmino");
        }
    }
}
