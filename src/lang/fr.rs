// !! INCREASE THE NUMBER WHEN CREATING NEW ENTRIES! !! //
// -----------------------------V---------------------- //
pub static LANG: [(&str, &str); 10] = [
    ("appdata_not_found",       "Variable d'environnement APPDATA non définie !"),
    ("unsupported_os",          "Ce programme ne prend en charge que Windows, macOS et Linux."),

    ("update_start",            "Installation de la mise à jour en cours..."),
    ("copy_success",            "Installation terminée.\nNettoyage des fichiers temporaires de mise à jour en cours..."),
    ("cleanup_success",         "Nettoyage terminé.\nMise à jour effectuée avec succès."),
    ("launch_prompt",           "Démarrer Techmino ? [Y/N] : "),
    
    ("update_fail",             "Échec de la mise à jour."),
    ("copy_fail",               "Erreur lors de la copie : "),
    ("cleanup_fail",            "Erreur lors du nettoyage : "),
    ("launch_fail",             "Échec du lancement de Techmino."),
];