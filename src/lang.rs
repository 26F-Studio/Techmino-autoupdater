use std::collections::HashMap;
use crate::multiplatform;

mod en; mod es; mod fr; mod id; mod ja; mod pt; mod vt; mod symbol; mod zh; mod zh_trad;

static mut LANG: Option<HashMap<&'static str, &'static str>> = None;

pub fn init() {
    let language = multiplatform::get_lang();
    let lang_map = match language.as_str() {
        "es" => es::LANG.iter().cloned().collect(),
        "fr" => fr::LANG.iter().cloned().collect(),
        "id" => id::LANG.iter().cloned().collect(),
        "ja" => ja::LANG.iter().cloned().collect(),
        "pt" => pt::LANG.iter().cloned().collect(),
        "vt" => vt::LANG.iter().cloned().collect(),
        "zh" => zh::LANG.iter().cloned().collect(),
        "zh_trad" => zh_trad::LANG.iter().cloned().collect(),
        "zh_code" => zh::LANG.iter().cloned().collect(),
        "symbol" => symbol::LANG.iter().cloned().collect(),
        _ => en::LANG.iter().cloned().collect(), // always fall back to English
    };
    unsafe {
        LANG = Some(lang_map);
    }
}

pub fn get_entry<'a>(entry: &'a str) -> &'a str {
    unsafe {
        if let Some(lang_map) = &LANG {
            return lang_map.get(entry).map(|s| *s).unwrap();
        }
        return entry;
    }
}