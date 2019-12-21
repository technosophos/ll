use clap::{App, Arg};
use ll::*;
use std::os::unix::fs::PermissionsExt;
//use std::ffi::{OsStr, OsString};

use std::fs::read_dir;

const FILENAME_WIDTH: usize = 80;
//const MAX_WIDTH: i8 = 80;

type LLResult = Result<(), failure::Error>;

fn main() -> LLResult {
    let matches = App::new("A pretty version of 'ls'")
        .version("2.0.0")
        .author("Technosophos")
        .about("List the contents of a directory, with pretty icons and human-readable sizes and dates.")
        .arg(Arg::with_name("all").short("a").long("all").help("View all files, including hidden ones").takes_value(false))
        .arg(Arg::with_name("DIR").help("The directory to view").index(1))
        .get_matches();
    let dir = matches.value_of("DIR").unwrap_or("./").to_string();
    let all = matches.is_present("all");
    print_entries(dir, all)
}

//fn print_header() {}
//fn print_footer() {}
fn print_entries(dirname: String, show_hidden: bool) -> LLResult {
    let mut dirs: Vec<_> = read_dir(dirname)?.map(|r| r.unwrap()).collect();
    dirs.sort_by_key(|d| d.path());
    let icons = IconResolver::default();
    let mut width = 0;
    let mut lines = vec![];
    for entry in dirs {
        let file_name = entry.file_name();
        let fname = file_name.to_str().unwrap();
        if !show_hidden && fname.starts_with('.') {
            continue;
        }
        width = fname.len().max(width).min(FILENAME_WIDTH);
        let icon = icons.icon(&entry);
        let mode = entry.metadata()?.permissions().mode();
        lines.push(Entry {
            icon,
            name: fname.to_string(),
            bytes: entry.metadata()?.len(),
            modified: entry.metadata()?.modified()?,
            executable: mode & 0o100 != 0,
            mode: entry.metadata()?.permissions().mode(),
            dir: entry.file_type()?.is_dir(),
        });
    }
    for l in lines {
        println!(
            "{}  {: <width$}  {:>8} {} {:>6o}",
            l.pretty_icon(),
            l.name,
            l.pretty_size(),
            l.pretty_time(),
            l.mode,
            width = width
        );
    }
    Ok(())
}
