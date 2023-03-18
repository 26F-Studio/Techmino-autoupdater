// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "Variável de ambiente APPDATA não definida!"),
    ("unsupported_os",          "Este programa suporta apenas Windows, macOS e Linux."),

    ("update_start",            "Instalando atualização..."),
    ("copy_success",            "Instalação concluída.\nLimpando arquivos temporários de atualização..."),
    ("cleanup_success",         "Limpeza concluída.\nAtualização concluída com êxito."),
    ("launch_prompt",           "Iniciar Techmino? [Y/N]: "),
    
    ("update_fail",             "Atualização falhou."),
    ("copy_fail",               "Erro ao copiar: "),
    ("cleanup_fail",            "Erro ao limpar: "),
    ("launch_fail",             "Falha ao iniciar Techmino."),
];