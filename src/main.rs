use std::fs;
use std::io::{self, Write};
use std::path::Path;

mod multiplatform;
mod lang;

// const SRC_WIN: &str = r"%appdata%\Techmino\updateExtract\";


fn main() -> std::io::Result<()> {
    lang::init();
    let src_buf = multiplatform::get_src();
    let src: &Path = src_buf.as_path();

    let dest_buf = std::env::current_exe().unwrap();
    let dest = dest_buf.parent().unwrap();

    println!("{}", lang::get_entry("update_start"));
    update(src, dest).expect(lang::get_entry("update_fail"));

    loop {
        print!("{}", lang::get_entry("launch_prompt"));
        
        io::stdout().flush().unwrap(); // Make sure prompt is displayed immediately
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap(); // get input from user

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                multiplatform::start_techmino();
                break;
            },
            "n" | "no" => {
                break;
            },
            _ => { continue; } // continue prompting the user for a valid input
        }
    }

    Ok(())
}

fn update(src: &Path, dest: &Path) -> io::Result<()> {
    match copy_dir(&src, &dest) {
        Err(e) => {println!("{}{}", lang::get_entry("copy_fail"), e); return Err(e)},
        Ok(_) => {println!("{}", lang::get_entry("copy_success"))}
    }
    match std::fs::remove_dir_all(src) {
        Err(e) => {println!("{}{}", lang::get_entry("cleanup_fail"), e); return Err(e)},
        Ok(_) => {println!("{}", lang::get_entry("cleanup_success"))}
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
