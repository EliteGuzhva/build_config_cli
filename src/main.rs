use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Launch {
    exe: String,
    #[serde(default)]
    cwd: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    before_script: String,
    #[serde(default)]
    env: Vec<String>,
}

impl Default for Launch {
    fn default() -> Self {
        Launch {
            exe: String::from(""),
            cwd: String::from("."),
            args: vec![],
            before_script: String::from(""),
            env: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Python {
    #[serde(default)]
    script: String,
    #[serde(default)]
    cwd: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    exe: String,
    #[serde(default)]
    venv: Option<String>,
}

impl Default for Python {
    fn default() -> Self {
        Python {
            cwd: String::from("."),
            exe: String::from("python3"),
            script: String::from("main.py"),
            args: vec![],
            venv: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Cargo {
    #[serde(default)]
    bin: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    exe: String,
    #[serde(default)]
    install_dir: String,
}

impl Default for Cargo {
    fn default() -> Self {
        Cargo {
            bin: String::from("src/main.rs"),
            args: vec![],
            exe: String::from("cargo"),
            install_dir: String::from("install"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BuildConfig {
    #[serde(default)]
    build_dir: String,
    #[serde(default)]
    launch: Launch,
    #[serde(default)]
    python: Python,
    #[serde(default)]
    cargo: Cargo,
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = ".build_config.json";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let config: BuildConfig = serde_json::from_reader(reader)?;

    println!("{:#?}", config.cargo);

    let mut cmd = Command::new(config.cargo.exe);
    cmd.arg("build");
    println!("{:#?}", cmd);
    let output = cmd.output().expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    // println!("{}", String::from_utf8(output.stdout).unwrap());

    Ok(())
}
