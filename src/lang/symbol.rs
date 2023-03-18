// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "X  APPDATA ENV_VAR"),
    ("unsupported_os",          "X  WIN/MAC/LINUX"),

    ("update_start",            "+  UPDATE"),
    ("copy_success",            "+  CLEANUP"),
    ("cleanup_success",         "V  CLEANUP, UPDATE"),
    ("launch_prompt",           "?  LAUNCH [Y/N]: "),
    
    ("update_fail",             "X  UPDATE"),
    ("copy_fail",               "X  COPY: "),
    ("cleanup_fail",            "X  CLEANUP: "),
    ("launch_fail",             "X  LAUNCH"),
];