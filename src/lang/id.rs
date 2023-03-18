// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "Variabel lingkungan APPDATA tidak diatur!"),
    ("unsupported_os",          "Program ini hanya mendukung Windows, macOS, dan Linux."),

    ("update_start",            "Menginstal pembaruan..."),
    ("copy_success",            "Instalasi selesai.\nMembersihkan file sementara..."),
    ("cleanup_success",         "Pembersihan selesai.\nPembaruan berhasil."),
    ("launch_prompt",           "Buka Techmino? [Y/N]: "),
    
    ("update_fail",             "Gagal memperbarui."),
    ("copy_fail",               "Terjadi kesalahan saat menyalin: "),
    ("cleanup_fail",            "Terjadi kesalahan saat membersihkan: "),
    ("launch_fail",             "Gagal meluncurkan Techmino."),
];