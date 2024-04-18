use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

const CREATE_NO_WINDOW: u32 = 0x08000000;
const VENV_DIR: &str = ".venv";
pub fn create_venv() -> () {
    let path = std::env::current_dir()
        .unwrap()
        .join("scripts")
        .join("create_venv.bat");

    let out = Command::new(path)
        .spawn()
        .expect("failed to create python venv");

    println!("Output: {:?}", out.stdout);
}

pub fn run_script(script_path: &str, args: Vec<String>) -> ExitStatus {
    let venv_path = if cfg!(windows) {
        Path::new(VENV_DIR)
            .join("Scripts")
            .join("activate.bat")
    } else {
        Path::new(VENV_DIR)
            .join("bin")
            .join("activate")
    };

    let activate_script = venv_path.to_str().unwrap();

    let mut values = Vec::new();

    values.extend(args.iter().map(|s| s.to_string()));

    let mut command_args = vec!["/C", &activate_script, "&&", "python", script_path];

    command_args.extend(values.iter().map(|s| s.as_str()));

    let mut child = Command::new("cmd")
        .args(&command_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("Failed to execute Python script: {}", e))
        .unwrap();

    child.wait().expect("failed to wait on child")
}
