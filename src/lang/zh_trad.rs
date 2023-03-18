// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "APPDATA 環境變數未設定！"),
    ("unsupported_os",          "此程式僅支援 Windows、macOS 和 Linux。"),

    ("update_start",            "正在安裝更新..."),
    ("copy_success",            "安裝完成。\n正在清理暫存檔案..."),
    ("cleanup_success",         "清理完成。\n更新成功。"),
    ("launch_prompt",           "啟動 Techmino？[Y/N]: "),
    
    ("update_fail",             "更新失敗。"),
    ("copy_fail",               "複製時發生錯誤："),
    ("cleanup_fail",            "清理時發生錯誤："),
    ("launch_fail",             "啟動 Techmino 失敗。"),
];