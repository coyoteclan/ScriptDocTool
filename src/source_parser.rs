// Regex patterns are AI generated

use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::read_to_string;
use std::{io, io::Write};
use termsize;

use crate::{CLEAR_COLOR, BHI_WHITE, B_YELLOW, B_CYAN};

#[derive(Serialize, Debug)]
pub(crate) struct ParseResult {
    // BTreeMap arranges entries in alphabetical order
    pub functions: BTreeMap<String, BTreeMap<String, ScriptFunction>>,
    pub methods: BTreeMap<String, BTreeMap<String, ScriptFunction>>,
}

#[derive(Serialize, Debug)]
pub(crate) struct ScriptFunction {
    #[serde(skip_serializing)]
    pub name: String,
    #[serde(rename = "scriptName")]
    pub script_name: String,
    pub params: Option<Vec<ScriptParameter>>,
    pub returns: Vec<String>,   // e.g., ["bool", "undefined"]
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct ScriptParameter {
    pub param_type: String,
    pub param_name: String,
}

#[derive(Debug)]
pub(crate) struct ScriptFunctionDetails {
    pub params: Option<Vec<ScriptParameter>>,
    pub returns: Vec<String>,
}

/*fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}*/

pub(crate) fn parse() -> io::Result<ParseResult>
{
    // Parse gsc.cpp to get script names and function names
    println!("{BHI_WHITE}Reading {B_CYAN}gsc.cpp{CLEAR_COLOR}");
    let (script_functions, script_methods) = parse_gsc_cpp("src/gsc/gsc.cpp")?;

    // Combine with function details from gsc_{category}.cpp files
    let mut final_functions: BTreeMap<String, BTreeMap<String, ScriptFunction>> = BTreeMap::new();
    let mut final_methods: BTreeMap<String, BTreeMap<String, ScriptFunction>> = BTreeMap::new();

    println!("{BHI_WHITE}Parsing Script Functions{CLEAR_COLOR}");
    for (category, functions) in script_functions {
        //println!("Category: {}", &category);
        //println!("Functions: {:#?}", functions);
        //std::process::exit(0);
        let category_file = format!("src/gsc/gsc_{}.cpp", category);
        let function_details = parse_category_file(&category_file)?;

        for func in functions {
            if let Some(details) = function_details.get(&func.name) {
                let script_func = ScriptFunction {
                    name: func.name.clone(),
                    script_name: func.script_name.clone(),
                    params: Some(details.params.clone().expect("failed to get params")),
                    returns: details.returns.clone(),
                };
                final_functions
                    .entry(category.clone()).or_default()
                    .insert(func.name.clone(), script_func);
            }
        }
    }
    let termsize::Size {cols, ..} = termsize::get().unwrap();
    let mut outputstr = String::new();
    for _ in 0..cols {
        outputstr.push(' ');
    }
    print!("\r{}", outputstr);

    let mut total_funcs: u16 = 0;
    for (category, funcs) in &final_functions {
        println!("{B_YELLOW}{}{CLEAR_COLOR} functions in {B_CYAN}gsc_{}.cpp{CLEAR_COLOR}", funcs.len(), category);
        total_funcs = total_funcs
            .checked_add(funcs.len() as u16)
            .expect("too many functions to fit in u16");
    }
    println!("Total {B_YELLOW}{}{CLEAR_COLOR} script functions", total_funcs);

    //println!("script_methods @68: {:#?}", script_methods);
    println!("\n{BHI_WHITE}Parsing Script Methods{CLEAR_COLOR}");
    for (category, methods) in script_methods {
        //println!("category @70: {}", &category);

        let category_file = format!("src/gsc/gsc_{}.cpp", category);
        //println!("category_file @73: {}", &category_file);
        let function_details = parse_category_file(&category_file)?;

        //println!("function_details @75: {:#?}", function_details);

        for meth in methods {
            if let Some(details) = function_details.get(&meth.name) {
                let script_meth = ScriptFunction {
                    name: meth.name.clone(),
                    script_name: meth.script_name.clone(),
                    params: Some(details.params.clone().expect("failed to get params")),
                    returns: details.returns.clone(),
                };
                final_methods
                    .entry(category.clone()).or_default()
                    .insert(meth.script_name.clone(), script_meth);
            }
        }
    }
    let termsize::Size {cols, ..} = termsize::get().unwrap();
    let mut outputstr = String::new();
    for _ in 0..cols {
        outputstr.push(' ');
    }
    print!("\r{}", outputstr);

    let mut total_meths: u16 = 0;
    for (category, meths) in &final_methods {
        println!("{B_YELLOW}{}{CLEAR_COLOR} methods in {B_CYAN}gsc_{}.cpp{CLEAR_COLOR}", meths.len(), category);
        total_meths = total_meths
            .checked_add(meths.len() as u16)
            .expect("too many functions to fit in u16");
    }
    println!("Total {B_YELLOW}{}{CLEAR_COLOR} script methods\n", total_meths);

    // Output as JSON
    //print_type_of(&final_data);
    //println!("{}", serde_json::to_string_pretty(&final_data).unwrap());
    Ok(ParseResult {
        functions: final_functions,
        methods: final_methods,
    })
}

// Parse gsc.cpp to extract script names and function names
fn parse_gsc_cpp(file_path: &str) -> io::Result<(
    HashMap<String, Vec<ScriptFunction>>,
    HashMap<String, Vec<ScriptFunction>>,
)>
{
    let code = read_to_string(file_path).expect("Failed to read gsc.cpp");
    let mut functions: HashMap<String, Vec<ScriptFunction>> = HashMap::new();
    let mut methods: HashMap<String, Vec<ScriptFunction>> = HashMap::new();
    let mut current = None;

    let line_re = Regex::new(r#"\{\s*"([^"]+)"\s*,\s*(\w+)\s*,\s*\d+\s*\},"#).unwrap();

    for line in code.lines() {
        let sline = line.trim();
        if sline.starts_with("//") { continue; }
        if sline.contains("scriptFunctions[]") {
            current = Some("functions");
            continue;
        } else if sline.contains("scriptMethods[]") {
            current = Some("methods");
            //println!("current: methods");
            continue;
        }
        if current.is_none() || sline.contains("test") {
            continue;
        }
        if let Some(caps) = line_re.captures(sline) {
            let script_name = caps[1].to_string();
            let func_name = caps[2].to_string();
            let parts: Vec<&str> = func_name.split('_').collect();
            if parts.len() >= 2 && parts[0] == "gsc" {
                let category = parts[1].to_string();
                let script_func = ScriptFunction {
                    name: func_name,
                    script_name,
                    params: None,
                    returns: vec![],
                };
                match current {
                    Some("functions") => functions.entry(category).or_default().push(script_func),
                    Some("methods") => methods.entry(category).or_default().push(script_func),
                    _ => {}
                }
            }
        }
    }
    Ok((functions, methods))
}

fn parse_category_file(file_path: &str) -> io::Result<HashMap<String, ScriptFunctionDetails>>
{
    let code = read_to_string(file_path).expect("Failed to read category file");
    let mut details_map: HashMap<String, ScriptFunctionDetails> = HashMap::new();

    let func_signature_re = Regex::new(r"(?m)void\s+(\w+)\s*\([^)]*\)\s*(?://[^\n]*)?\s*\{").unwrap();
    //println!("parse_category_file: {}", file_path.to_string());

    for cap in func_signature_re.captures_iter(&code) {
        let function_name = cap[1].to_string();
        let start = cap.get(0).unwrap().end();
        let mut brace_count = 1;
        let mut body = String::new();
        let mut i = start;

        let termsize::Size {cols, ..} = termsize::get().unwrap();
        let numspaces: u16 = (cols - function_name.chars().count() as u16) - 3 as u16;
        let mut outputstr = format!("\r{}...", &function_name);
        for _ in 1..numspaces {
            outputstr.push(' ');
        }
        print!("{}", outputstr);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));

        while i < code.len() && brace_count > 0 {
            let c = code.as_bytes()[i] as char;
            body.push(c);
            if c == '{' {
                brace_count += 1;
            } else if c == '}' {
                brace_count -= 1;
            }
            i += 1;
        }
        if !body.is_empty() {
            body.pop(); // Remove closing brace
        }

        //println!("body: \n{}", &body);

        let params = extract_params(&body);
        //println!("{} params: {:#?}", &function_name, params.clone().expect("failed to print params"));
        //std::process::exit(0);
        let returns = extract_return_types(&body)?;

        details_map.insert(
            function_name,
            ScriptFunctionDetails {
                params,
                returns,
            },
        );
    }
    Ok(details_map)
}

// Extract parameter types from stackGetParams
fn extract_params(body: &str) -> Option<Vec<ScriptParameter>>
{
    let params_re = Regex::new(r#"\s*stackGetParams\(\s*"([^"]+)"\s*,([^)]+)\)"#).unwrap();
    if let Some(caps) = params_re.captures(body) {
        let param_types: Vec<String> = caps[1].chars().map(|s| {
            match s {
                'i' => "int".to_string(),
                'v' => "vector".to_string(),
                'f' => "float".to_string(),
                's' => "string".to_string(),
                'c' => "const string".to_string(),
                'l' => "localized string".to_string(),
                _ => "unknown".to_string(),
            }
        }).collect();
        let param_names: Vec<String> = caps[2]
            .split(',')
            .map(|s| s.trim().trim_start_matches('&').to_string())
            .collect();
        
        let params: Vec<ScriptParameter> = param_types.into_iter()
            .zip(param_names.into_iter())
            .map(|(param_type, param_name)| {
                ScriptParameter { param_type, param_name }
            })
            .collect();
        Some(params)
    } else {
        Some(vec![ ScriptParameter {
            param_type: "unknown".to_string(),
            param_name: "unknown".to_string()
        } ].into())
    }
}

fn extract_return_types(body: &str) -> io::Result<Vec<String>>
{
    let re = Regex::new(r"Scr_Add(\w+)\s*\(").unwrap();
    let mut return_types = HashSet::new();

    for cap in re.captures_iter(body) {
        let func_name = format!("Scr_Add{}", &cap[1]);
        if let Some(return_type) = map_scr_add_to_type(&func_name) {
            return_types.insert(return_type);
        }
    }

    if return_types.is_empty() {
        Ok(vec!["unknown".to_string()])
    } else {
        Ok(return_types.into_iter().collect())
    }
}

fn map_scr_add_to_type(func_name: &str) -> Option<String>
{
    match func_name {
        "Scr_AddBool" => Some("bool".to_string()),
        "Scr_AddInt" => Some("int".to_string()),
        "Scr_AddFloat" => Some("float".to_string()),
        "Scr_AddString" => Some("string".to_string()),
        "Scr_AddArray" => Some("array".to_string()),
        "Scr_AddVector" => Some("vector".to_string()),
        "Scr_AddObject" => Some("object".to_string()),
        //"Scr_AddUndefined" => Some("undefined (on failure)".to_string()),
        _ => None,//Some("unknown".to_string()),
    }
}
