use std::collections::HashMap;

mod en;
mod zh;

static mut LANG: Option<HashMap<&'static str, &'static str>> = None;

pub fn init() {
    let language = "en"; // TODO: get language from config
    let lang_map = match language {
        "zh" => zh::LANG.iter().cloned().collect(),
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