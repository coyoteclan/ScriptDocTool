use std::fs;
use std::path::Path;
use crate::source_parser::{ParseResult, ScriptFunction};
use std::path::PathBuf;
use std::io;
use io::ErrorKind;

use crate::{CLEAR_COLOR, BHI_WHITE, B_GREEN, B_PURPLE, B_CYAN};

pub fn generate_docs(parse_result: &ParseResult, fail_missing: bool, no_write: bool, write_sep: bool) -> io::Result<()>
{
    let base_dir = Path::new("docs/source/pages/scripting");
    fs::create_dir_all(&base_dir)?;

    let skip_no_write = format!("{BHI_WHITE}Skipping writing to docs since {B_PURPLE}--no-write {BHI_WHITE}was given.{CLEAR_COLOR}");
    let writing_sep = format!("{BHI_WHITE}Writing to a separate temp file since {B_PURPLE}--write-sep {BHI_WHITE}argument was given.{CLEAR_COLOR}");

    for (category, funcs) in &parse_result.functions {
        let file_path = base_dir.join("functions").join(format!("{}.rst", category));

        if !file_path.exists() {
            fs::write(&file_path, "")?;
        }
        let content = fs::read_to_string(&file_path)?;
        let mut template = String::new();

        for (_func_key, func) in funcs {
            let sign = get_func_sign(&func.script_name);
            if !content.contains(&sign) {
                let missing_notice = format!("Missing doc for {B_GREEN}{}{CLEAR_COLOR} in {B_CYAN}{}.rst{CLEAR_COLOR}", &func.script_name, &category);
                if fail_missing {
                    return Err(io::Error::new(ErrorKind::NotFound, format!("No documentation found for {} in {}.rst", &func.script_name, &category)));
                }
                else {
                    println!("{missing_notice}");
                    println!("Adding stub, please edit before commiting.\n");
                }
                let func_temp = gen_template(func, false)?;
                template.push_str(&format!("{}", func_temp));
            }
        }

        if !template.is_empty() {
            if write_sep {
                println!("{writing_sep}");
                let name = template.trim_start().lines().nth(0).unwrap();
                let file_path = base_dir.join("functions").join(format!("{}.temp.rst", name));
                fs::write(&file_path, &template)?;
                continue;
            }
            else if no_write {
                println!("{skip_no_write}");
                println!("{}", &template);
                continue;
            }
            append_to_file(template, &file_path)?;
        }
    }

    for (category, meths) in &parse_result.methods {
        let file_path = base_dir.join("methods").join(format!("{}.rst", category));

        if !file_path.exists() {
            fs::write(&file_path, "")?;
        }
        let content = fs::read_to_string(&file_path)?;
        let mut template = String::new();

        for (_func_key, meth) in meths {
            let sign = get_func_sign(&meth.script_name);
            if !content.contains(&sign) {
                let missing_notice = format!("Missing doc for {B_GREEN}{}{CLEAR_COLOR} in {B_CYAN}{}.rst{CLEAR_COLOR}", &meth.script_name, &category);
                if fail_missing {
                    return Err(io::Error::new(ErrorKind::NotFound, format!("No documentation found for {} in {}.rst", &meth.script_name, &category)));
                }
                else {
                    println!("{missing_notice}");
                    println!("Adding stub, please edit before commiting.\n");
                }
                let meth_temp = gen_template(meth, true)?;
                template.push_str(&format!("{}", meth_temp));
            }
        }

        if !template.is_empty() {
            if write_sep {
                println!("{writing_sep}");
                let name = template.trim_start().lines().nth(0).unwrap();
                let file_path = base_dir.join("functions").join(format!("{name}.temp.rst"));
                fs::write(&file_path, &template)?;
                continue;
            }
            else if no_write {
                println!("{skip_no_write}");
                println!("{}", &template);
                continue;
            }
            append_to_file(template, &file_path)?;
        }
    }

    Ok(())
}

#[rustfmt::skip]
fn gen_template(func: &ScriptFunction, is_method: bool) -> io::Result<String>
{
    let mut current = "function".to_string();
    let mut calledon = String::new();
    if is_method {
        current = "method".to_string();
        calledon = "<some object> ".to_string();
    }
    let mut template = String::new();

    //for (_func_key, func) in funcs {
        // detect existing entry by scriptName directive
        let sign = get_func_sign(&func.script_name);

        template.push_str(&format!(r#"
{sign}

.. csv-table:: **Arguments**
    :header: "Argument", "Type", "Description"
    :align: left

"#));

        let mut args = String::new();
        let mut param_names: Vec<&str> = Vec::new();
        if let Some(params) = &func.params {
            for p in params {
                template.push_str(&format!("    \"{}\", \"{}\", \"description\"\n", p.param_name, p.param_type));
                param_names.push(&p.param_name)
            }
        }
        args.push_str(&param_names.join(", "));

        if is_method {
            template.push_str("\n| **Called on** ``<some object>``\n");
        }
        else {
            template.push_str("\n");
        }
        for r in &func.returns {
            template.push_str(&format!("| **Returns** ``{}``\n", r));
        }
        template.push_str(&format!("\nthis is the Description of the {}. Explain the usage in detail here\n", &current));
        template.push_str(&format!(r#"
**Example**

.. code-block:: cpp
    
    // stub example for dev.
    // dev. should remove this comment after he is done changing it
    {}({});

"#, &format!("{}{}", &calledon, &func.script_name), &args));

    //}

    Ok(template)
}

fn get_func_sign(name: &str) -> String
{
    format!("{}\n{}", name, "-".repeat(name.len()))
}

fn append_to_file(data: String, file_path: &PathBuf) -> io::Result<()>
{
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(file_path)?;
    use std::io::Write;
    writeln!(file, "\n{}", data)?;

    Ok(())
}
