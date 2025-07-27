mod source_parser;
mod doc_gen;
use source_parser::ParseResult;
use doc_gen::generate_docs;
//use std::collections::HashMap;

fn main() {
    //println!("Free Palestine 🍉️ 🇵🇸️ \n\n");
    let data: ParseResult = source_parser::parse();

    println!("\nsleeping");
    //std::thread::sleep(std::time::Duration::from_millis(3000));
    //println!("Functions: \n{}", serde_json::to_string_pretty(&data.functions).unwrap());
    //println!("Methods: \n{}", serde_json::to_string_pretty(&data.methods).unwrap());
    generate_docs(&data).unwrap();
}
