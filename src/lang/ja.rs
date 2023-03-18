// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "APPDATA環境変数が設定されていません！"),
    ("unsupported_os",          "このプログラムはWindows、macOS、Linuxのみをサポートしています。"),

    ("update_start",            "アップデートをインストール中..."),
    ("copy_success",            "インストールが完了しました。\n一時的な更新ファイルをクリーンアップしています..."),
    ("cleanup_success",         "クリーンアップが完了しました。\nアップデートが正常に完了しました。"),
    ("launch_prompt",           "Techminoを起動しますか？ [Y/N]: "),
    
    ("update_fail",             "アップデートに失敗しました。"),
    ("copy_fail",               "コピー中にエラーが発生しました: "),
    ("cleanup_fail",            "クリーンアップ中にエラーが発生しました: "),
    ("launch_fail",             "Techminoの起動に失敗しました。"),
];