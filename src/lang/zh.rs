// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "未设置APPDATA环境变量！"),
    ("unsupported_os",          "该程序仅支持Windows、macOS和Linux。"),

    ("update_start",            "正在安装更新..."),
    ("copy_success",            "安装完成。\n正在清理临时更新文件..."),
    ("cleanup_success",         "清理完成。\n更新已成功完成。"),
    ("launch_prompt",           "启动Techmino？ [Y/N]: "),
    
    ("update_fail",             "更新失败。"),
    ("copy_fail",               "复制时出错："),
    ("cleanup_fail",            "清理时出错："),
    ("launch_fail",             "无法启动Techmino。"),
];