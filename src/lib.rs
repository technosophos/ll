use chrono::prelude::*;
use colorful::{Color, Colorful};
use size_format::SizeFormatterSI;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::time::UNIX_EPOCH;

pub struct Entry {
    pub icon: String,
    pub name: String,
    pub bytes: u64,
    pub mode: u32,
    pub modified: std::time::SystemTime,
    pub executable: bool,
    pub dir: bool,
}

impl Entry {
    /// Read bytes and convert to a pretty size.
    pub fn pretty_size(&self) -> String {
        format!("{}B", SizeFormatterSI::new(self.bytes))
    }
    pub fn pretty_time(&self) -> String {
        Local
            .timestamp_millis(
                self.modified
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|_| std::time::Duration::new(0, 0))
                    .as_millis() as i64,
            )
            .format("   %D    %R")
            .to_string()
    }
    pub fn pretty_icon(&self) -> String {
        if self.executable {
            let ico = if self.icon == DEFAULT_ICON {
                //"\u{fc0c}".to_string() // Running man
                "\u{f96d}".to_string()
            } else {
                self.icon.clone()
            };
            return ico.color(Color::Red1).to_string();
        }
        self.icon.clone()
    }
}

pub struct IconResolver {
    file_icons: HashMap<String, String>,
    ext_icons: HashMap<String, String>,
    dir_icons: HashMap<String, String>,
}

const DEFAULT_ICON: &str = "\u{f036}";
const FOLDER_ICON: &str = "\u{f07b}";
const LINK_ICON: &str = "\u{f481}";

impl IconResolver {
    pub fn icon(&self, entry: &std::fs::DirEntry) -> String {
        let file_name = entry.file_name();
        let n = file_name.to_str().unwrap();
        if let Ok(ft) = entry.file_type() {
            if ft.is_dir() {
                return self
                    .dir_icons
                    .get(n)
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| FOLDER_ICON.to_string())
                    .color(Color::Yellow)
                    .to_string();
            }
            if ft.is_symlink() {
                return LINK_ICON.to_string().color(Color::Aquamarine1a).to_string();
            }
        }
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

/// Build a map icons mapped to file extensions or file names.
macro_rules! icomap {
    (
        $(
            $icon:expr => [ $($i: expr),* ]
        ),*
    ) => {
        {
            let mut icons: HashMap<String, String> = HashMap::new();
            $(
                $(
                    icons.insert($i.to_string(), $icon.to_string());
                )*
            )*
            icons
        }
    };
}

impl Default for IconResolver {
    fn default() -> Self {
        let file_icons = icomap![
            "\u{f113}" => [".gitignore"],
            "\u{e20f}" => ["Makefile", "justfile"],
            "\u{e7b0}" => ["Dockerfile"],
            "\u{f0c3}" => ["brigade.js"],
            "\u{f1d8}" => ["glide.yaml", "Gopkg.toml"],
            "\u{f6f6}" => ["go.mod", "go.sum"],
            "\u{e718}" => ["package.json"],
            "\u{e711}" => [".DS_Store"],
            "\u{f071}" => ["LICENSE", "COPYING", "LICENSE", "license.txt", "LICENSE.txt", "COPYING.txt"],
            "\u{f8d6}" => ["Cargo.toml"],
            "\u{f120}" => [".bashrc", ".zshrc", ".profile"],
            "\u{f15c}" => ["NOTES.txt", "notes.txt"],
            "\u{f0cb}" => ["TODO.txt"],
            "\u{fad0}" => ["code-of-conduct.md", "conduct.md", "conduct.txt"],
            "\u{f278}" => ["CONTRIBUTING", "CONTRIBUTING.md"],
            "\u{f0c0}" => ["CODEOWNERS", "OWNERS"],
            "\u{f1ea}" => ["README.md", "README.txt", "README"]
        ];

        let ext_icons = icomap![
            "\u{e7a8}" => ["rs"],
            "\u{f023}" => ["lock"],
            "\u{f120}" => ["sh", "bash", "zsh", "ksh"],
            "\u{e781}" => ["js"],
            "\u{f18d}" => ["log"],
            "\u{f19e}" => ["yaml", "yml"],
            "\u{e60b}" => ["json"],
            "\u{f085}" => ["conf", "config", "cfg"],
            "\u{f1b3}" => ["bin"],
            "\u{fab2}" => ["exe"],
            "\u{f1e2}" => ["err"],
            "\u{fbe4}" => ["ts"],
            "\u{e73c}" => ["py", "pyc", "pyo"],
            "\u{e609}" => ["md", "markdown"],
            "\u{e739}" => ["rb"],
            "\u{e738}" => ["java", "class"],
            "\u{e626}" => ["go"],
            "\u{f1c9}" => ["html", "htm", "xhtml"],
            "\u{e749}" => ["css"],
            "\u{e758}" => ["less"],
            "\u{e74b}" => ["scss", "sass"],
            "\u{e73d}" => ["php", "phar"],
            "\u{e60e}" => ["tpl", "tmpl"],
            "\u{e61b}" => ["coffee"],
            "\u{f1c6}" => ["tgz", "gz", "zip", "bz2", "bz"],
            "\u{f1c1}" => ["pdf"],
            "\u{e60d}" => ["png", "gif", "jpg", "jpeg", "ico", "svg", "tiff", "bmp"],
            "\u{f1c2}" => ["doc", "docx", "odt"],
            "\u{f1c4}" => ["ppt", "pptx"],
            "\u{f1c3}" => ["xls", "xlsx"],
            "\u{f0ce}" => ["csv", "tsv", "tab"],
            "\u{f084}" => ["pgp", "gpg", "gpg~", "kbx", "kbx~"],
            "\u{f1c0}" => ["db", "sqlite", "sqlite3"],
            "\u{f008}" => ["mov", "mp4"],
            "\u{f0fa}" => ["backup", "old", "OLD"],
            "\u{e7c5}" => ["vim"],
            "\u{e20f}" => ["mk", "make"],
            "\u{f0fd}" => ["h"],
            "\u{e61e}" => ["c"],
            "\u{e61d}" => ["cc", "c++"],
            "\u{f669}" => ["toml", "tml"],
            "\u{e7b1}" => ["ex", "exs"],
            "\u{f0f6}" => ["txt"],
            "\u{e712}" => ["so"],
            "\u{e711}" => ["dylib"],
            "\u{e704}" => ["sql", "mysql", "msql"],
            "\u{e76e}" => ["pgsql", "psql"],
            "\u{e737}" => ["scala"],
            "\u{e713}" => ["applescript"],
            "\u{f085}" => ["wasm"], //Cogs
            "\u{f253}" => ["tmp", "temp"],
            "\u{f623}" => ["crt", "csr"],
            "\u{f805}" => ["key"],
            "\u{f736}" => ["pem", "pub"],
            "\u{f81a}" => ["cs"],
            "\u{f840}" => ["asc"],
            "\u{fafa}" => ["sha", "sha256", "sha512"],
            "\u{f499}" => ["test"],
            "\u{f977}" => ["script"],
            "\u{e620}" => ["lua"]
        ];

        let dir_icons = icomap![
            "\u{e5fb}" => [".git"],
            "\u{f74e}" => ["Pictures", "Movies"],
            "\u{e713}" => ["Applications", "Library"],
            "\u{e780}" => ["Code"],
            "\u{e238}" => ["_scratch"],
            "\u{fd03}" => [".azure"],
            "\u{fadc}" => ["target"],
            "\u{f64e}" => ["Documents"],
            "\u{fcbe}" => ["Desktop"],
            "\u{f6d9}" => ["Downloads"],
            "\u{f832}" => ["Music"],
            //"\u{fb27}" => ["bin"],
            "\u{fd24}" => ["Users", "home"],
            "\u{f499}" => ["test", "testdata"],
            "\u{fd2f}" => ["deis", "deislabs"],
            "\u{fd31}" => ["charts", "helm"],
            "\u{f482}" => ["scripts", "hack"],
            "\u{f414}" => ["pkg", "lib"],
            "\u{f43f}" => ["vendor", "node_modules"]
        ];

        IconResolver {
            file_icons,
            ext_icons,
            dir_icons,
        }
    }
}
