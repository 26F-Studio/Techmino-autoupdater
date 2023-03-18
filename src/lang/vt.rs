// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "Biến môi trường APPDATA không được thiết lập!"),
    ("unsupported_os",          "Chương trình này chỉ hỗ trợ Windows, macOS và Linux."),

    ("update_start",            "Đang cài đặt cập nhật..."),
    ("copy_success",            "Cài đặt hoàn tất.\nĐang dọn dẹp các tệp cập nhật tạm thời..."),
    ("cleanup_success",         "Dọn dẹp hoàn tất.\nCập nhật đã hoàn tất thành công."),
    ("launch_prompt",           "Bắt đầu Techmino? [Y/N]: "),
    
    ("update_fail",             "Cập nhật thất bại."),
    ("copy_fail",               "Lỗi khi sao chép: "),
    ("cleanup_fail",            "Lỗi khi dọn dẹp: "),
    ("launch_fail",             "Không thể khởi chạy Techmino."),
];