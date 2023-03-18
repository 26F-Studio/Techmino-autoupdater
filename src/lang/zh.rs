// !! INCREASE THE NUMBER WHEN CREATING NEW ENTIRES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "APPDATA environment variable not set!"),
    ("unsupported_os",          "This program only supports Windows, macOS, and Linux."),

    ("update_start",            "Installing update..."),
    ("copy_success",            "Installation complete.\nCleaning up temporary update files..."),
    ("cleanup_success",         "Cleanup complete.\nUpdate completed successfully."),
    ("launch_prompt",           "Start Techmino? [Y/N]: "),
    
    ("update_fail",             "Update failed."),
    ("copy_fail",               "Error while copying: "),
    ("cleanup_fail",            "Error while cleaning up: "),
    ("launch_fail",             "Failed to launch Techmino."),
];