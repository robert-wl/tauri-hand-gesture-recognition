use std::io::{BufRead, BufReader};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};

use tauri::Manager;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const VENV_DIR: &str = ".venv";

pub fn run_script(script_path: &Path, args: Vec<String>) -> Child {
    let venv_path = if cfg!(windows) {
        Path::new(VENV_DIR).join("Scripts").join("activate.bat")
    } else {
        Path::new(VENV_DIR).join("bin").join("activate")
    };

    let activate_script = venv_path.to_str().unwrap();

    let mut values = Vec::new();

    values.extend(args.iter().map(|s| s.to_string()));

    let string_path = script_path.to_str().unwrap();

    let mut command_args = vec!["/C", &activate_script, "&&", "python", string_path];

    command_args.extend(values.iter().map(|s| s.as_str()));

    Command::new("cmd")
        .args(&command_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("Failed to execute Python script: {}", e))
        .unwrap()
}
