use handlebars::Handlebars;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use serde_json::Map as JsonMap;
use serde_json::value::Value as JsonValue;
use serde_yaml::Mapping as YamlMap;
use clap::clap_app;

fn read_vars_from_json_file<P: AsRef<Path>>(path: P) -> Result<JsonMap<String, JsonValue>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let vars = serde_json::from_reader(reader)?;

    Ok(vars)
}

fn read_vars_from_yaml_file<P: AsRef<Path>>(path: P) -> Result<YamlMap, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let vars = serde_yaml::from_reader(reader)?;

    Ok(vars)
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Jérôme Arzel <jerome@quitoque.fr>")
        (about: "Tera CLI")
        (@arg VARIABLES_FILE: +required "Variables file (JSON format).")
        (@arg TEMPLATE_FILE: +required "Handlebars template file")
    ).get_matches();

    let vars_file_path = matches.value_of("VARIABLES_FILE").unwrap();
    let template_path = matches.value_of("TEMPLATE_FILE").unwrap();

    let mut tpl_str = String::new();
    let mut file = File::open(template_path)?;

    file.read_to_string(&mut tpl_str)?;

    let reg = Handlebars::new();

    if vars_file_path.ends_with(".json") {
        let vars = read_vars_from_json_file(vars_file_path)?;

        println!("{}", reg.render_template(&tpl_str, &vars)?)
    } else {
        let vars = read_vars_from_yaml_file(vars_file_path)?;

        println!("{}", reg.render_template(&tpl_str, &vars)?)
    }

    Ok(())
}
