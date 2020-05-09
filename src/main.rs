use structopt::StructOpt;
use serde::{Deserialize};
use serde_yaml;
use std::fs;
use regex::{Regex,NoExpand,Captures};
use std::collections::HashMap;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to configuration file.
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Deserialize)]
struct Whalepod {
    versions: Vec<String>,
    template: String,
    parameters: HashMap<String, String>
}

fn main() -> Result<(), serde_yaml::Error> {
    let args = Cli::from_args();
    let content = fs::read_to_string(&args.path).unwrap();
    let d: Whalepod = serde_yaml::from_str(&content)?;

    // Setup regex.
    let version_reg = Regex::new("<{2}(version)>{2}").unwrap();
    let param_regex = Regex::new("<{2}(params.)(?P<param_name>.*?)>{2}").unwrap();

    for version in d.versions {
        fs::create_dir(&version).ok();
        let tmpl_with_version = version_reg.replace_all(&d.template, NoExpand(&version));
        let parameters = &d.parameters;
        let tmpl = param_regex.replace_all(&tmpl_with_version, |cap: &Captures| {
            let param = parameters.get(&cap["param_name"]).unwrap();
            return param;
        });

        let _file = fs::write(format!("{}/Dockerfile", &version), tmpl.as_bytes()).expect("unable to open");
    }
    Ok(())
}
