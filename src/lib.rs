#[macro_use]
extern crate maplit;

use maplit::hashmap;
use std::collections::HashMap;
use std::env::args;
use std::ffi::{OsStr, OsString};
use std::fs::read_dir;

pub struct IconResolver {
    file_icons: HashMap<String, String>,
    ext_icons: HashMap<String, String>,
}

const DEFAULT_ICON: &str = "\u{f036}";
const FOLDER_ICON: &str = "\u{f07b}";
const LINK_ICON: &str = "\u{f481}";

impl IconResolver {
    pub fn new() -> Self {
        let file_icons = hashmap![
            ".Dockerfile".into() => "\u{e7b0}".into(),
        ];
        let ext_icons = hashmap![
            "rs".into() => "\u{e7a8}".into(),
        ];
        IconResolver {
            file_icons,
            ext_icons,
        }
    }
    pub fn icon(&self, entry: &std::fs::DirEntry) -> String {
        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                return FOLDER_ICON.to_string();
            }
            if ft.is_symlink() {
                return LINK_ICON.to_string();
            }
        }
        let file_name = entry.file_name();
        let n = file_name.to_str().unwrap();
        match self.file_icons.get(n) {
            Some(icon) => icon.to_string(),
            None => {
                let ext = entry
                    .path()
                    .extension()
                    .unwrap_or_else(|| OsStr::new(""))
                    .to_str()
                    .unwrap()
                    .to_string();
                self.ext_icons
                    .get(&ext)
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| DEFAULT_ICON.to_string())
            }
        }
    }
}
