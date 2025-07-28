
const CLEAR_COLOR: &str = "\x1b[0m";
const BHI_WHITE: &str = "\x1b[1;97m";
const B_GREEN: &str = "\x1b[1;32m";
const B_YELLOW: &str = "\x1b[1;33m";
const B_PURPLE: &str = "\x1b[1;35m";
const B_CYAN: &str = "\x1b[1;36m";

mod source_parser;
mod doc_gen;
mod doc_sort;
use source_parser::ParseResult;
use doc_gen::generate_docs;
use doc_sort::sort;
use std::env;
use std::io;
use std::fs;
use std::path::Path;

fn main() -> io::Result<()>
{
    println!("{B_GREEN}Free Palestine{CLEAR_COLOR} 🍉️ 🇵🇸️ \n\n");
    let mut parse_only = false;
    let mut print_parsed = false;
    let mut fail_missing = false;
    let mut no_write = false;
    let mut write_sep = false;
    let mut sortdoc = false;

    for arg in env::args() {
        match arg.as_str() {
            "--parse-only" => parse_only = true,
            "--print-parsed" => print_parsed = true,
            "--fail-missing" => fail_missing = true,
            "--no-write" => no_write = true,
            "--write-sep" => write_sep = true,
            "--sort" => sortdoc = true,
            val if val == env::args().nth(0).unwrap().as_str() => continue,
            _ => eprintln!("Unknow argument: {}", arg),
        }
    }

    if sortdoc {
        let mut dirs: Vec<&Path> = Vec::new();
        dirs.push(Path::new("docs/source/pages/scripting/functions"));
        dirs.push(Path::new("docs/source/pages/scripting/methods"));

        for dir in dirs {
            let entries = fs::read_dir(dir)?;
            for entry in entries {
                let entry = entry?;
                let file_name = entry.file_name().to_string_lossy().to_string();
                if &file_name == "index.rst" {
                    println!("Skipping index.rst");
                    continue;
                }
                println!("{BHI_WHITE}Sorting {B_CYAN}{}{CLEAR_COLOR}", &file_name);
                sort(entry.path())?;
            }
        }
        println!("");

        return Ok(())
    }

    let data: ParseResult = source_parser::parse()?;

    if print_parsed {
        println!("Functions: \n{}", serde_json::to_string_pretty(&data.functions).unwrap());
        println!("Methods: \n{}", serde_json::to_string_pretty(&data.methods).unwrap());
        println!("");
    }

    if !parse_only {
        generate_docs(&data, fail_missing, no_write, write_sep).unwrap();
    }
    else {
        println!("{BHI_WHITE}Skipping doc generation since {B_PURPLE}--parse-only {BHI_WHITE}argument was given.{CLEAR_COLOR}");
    }

    Ok(())
}
