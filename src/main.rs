use std::env::args;
use std::ffi::{OsStr, OsString};
use std::fs::read_dir;

const FILENAME_WIDTH: i8 = 30;
const MAX_WIDTH: i8 = 80;

type LLResult = Result<(), failure::Error>;

fn main() -> LLResult {
    let dir = args().nth(1).unwrap_or_else(|| "./".into());
    print_entries(dir)
}

fn print_header() {}
fn print_footer() {}
fn print_entries(dirname: String) -> LLResult {
    for fi in read_dir(dirname)? {
        let icons = ll::IconResolver::new();
        let entry = fi?;
        let file_name = entry.file_name();
        let fname = file_name.to_str().unwrap();
        let icon = icons.icon(&entry);
        println!("{}  {}", icon, fname)
    }
    Ok(())
}
