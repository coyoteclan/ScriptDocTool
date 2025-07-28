use regex::Regex;
use std::{collections::BTreeMap, fs::read_to_string, fs::write, io, path::PathBuf};
use io::{Result, ErrorKind};

pub(crate) fn sort(file_path:PathBuf) -> Result<()>
{
    if !file_path.exists() {
        return Err(io::Error::new(ErrorKind::NotFound, format!("{} doesn't exist", file_path.to_str().unwrap())));
    }

    let rst_text = read_to_string(&file_path).expect("Failed to read rst file");

    let re = Regex::new(r"(?m)^(.+?)\n-{3,}\n").unwrap();

    let mut headers = Vec::new();
    for cap in re.captures_iter(&rst_text) {
        let function_name = cap.get(1).unwrap().as_str().to_string();
        //println!("{:#?}", cap);
        headers.push(function_name);
    }

    let pieces: Vec<&str> = re.split(&rst_text).collect();
    assert!(pieces.len() == headers.len() + 1, "Mismatch between headers and pieces");

    let title = pieces[0];

    // Build a BTreeMap to sort subsections alphabetically
    let mut subsections = BTreeMap::new();
    for (i, function_name) in headers.iter().enumerate() {
        subsections.insert(function_name.clone(), pieces[i + 1].to_string());
    }

    //println!("{}", serde_json::to_string_pretty(&subsections).unwrap());
    //std::process::exit(0);

    // Reconstruct the RST file
    let mut output = String::new();
    output.push_str(title);

    for (function_name, content) in &subsections {
        output.push_str(function_name);
        output.push('\n');
        let dashes = "-".repeat(function_name.len());
        output.push_str(&dashes);
        output.push('\n');
        output.push_str(content);
    }

    write(&file_path, &output)?;
    //println!("{}", &output);

    Ok(())
}

