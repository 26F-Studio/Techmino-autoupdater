// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "¡Variable de entorno APPDATA no definida!"),
    ("unsupported_os",          "Este programa solo admite Windows, macOS y Linux."),

    ("update_start",            "Instalando actualización..."),
    ("copy_success",            "Instalación completa.\nLimpiando archivos temporales de actualización..."),
    ("cleanup_success",         "Limpieza completa.\nActualización completada con éxito."),
    ("launch_prompt",           "¿Iniciar Techmino? [Y/N]: "),
    
    ("update_fail",             "Falló la actualización."),
    ("copy_fail",               "Error al copiar: "),
    ("cleanup_fail",            "Error al limpiar: "),
    ("launch_fail",             "No se pudo iniciar Techmino."),
];