use std::{fs, io};
use std::path::Path;

// const SRC_WIN: &str = r"%appdata%\Techmino\updateExtract\";
const SRC_MAC: &str = "/Users/user/Library/Application Support/LOVE/Techmino/updateExtract/";
const SRC_LNX: &str = "~/.local/share/love/Techmino/updateExtract";

fn main() -> std::io::Result<()> {
    let src;
    let src_buf; // unused by linux/mac. i trust the compiler that it'll optimize this away :)
    match std::env::consts::OS {
        "windows" => { 
            let appdata = std::env::var_os("APPDATA").expect("APPDATA environment variable not set");
            src_buf = Path::new(&appdata).join("Techmino").join("updateExtract");
            src = src_buf.as_path();
        }
        "macos"   => { src = Path::new(SRC_MAC); }
        "linux"   => { src = Path::new(SRC_LNX); }
        _ => panic!("Unsupported operating system: {}", std::env::consts::OS)
    }

    // we need this variable to exist separately for some reason. i won't be accessing it tho
    let __ = std::env::current_exe().unwrap();
    let dest = __.parent().unwrap();

    println!("Copying files from {} to {}... ", src.display(), dest.display());

    match copy_dir(&src, &dest) {
        Err(e) => {println!("Error while copying: {}", e); return Err(e)},
        Ok(_) => {println!("Copying complete.\n============\nCleaning up temporary update files...")}
    }
    
    match std::fs::remove_dir_all(src) {
        Err(e) => {println!("Error while cleaning up: {}", e); return Err(e)},
        Ok(_) => {println!("Cleanup complete.")}
    }

    Ok(())
}


fn copy_dir(from_dir: &Path, to_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(from_dir)? {
        let entry = entry?;

        let entry_path = entry.path();
        let new_path = to_dir.join(entry.file_name());
        // If the entry is a directory, recursively copy it to the destination
        if entry_path.is_dir() {
            fs::create_dir_all(&new_path)?;
            copy_dir(&entry_path, &new_path)?;
        }
        // If the entry is a file, copy it to the destination
        else {
            fs::copy(&entry_path, &new_path)?;
        }
    }

    Ok(())
}
