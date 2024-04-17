use std::fmt::Arguments;
use std::os::windows::process::CommandExt;
use std::process::{Command, ExitStatus, Stdio};




const CREATE_NO_WINDOW: u32 = 0x08000000;
const VENV_DIR: &str = ".venv";
pub fn create_venv() -> () {
    let path = std::env::current_dir()
        .unwrap()
        .join("scripts\\create_venv.bat");

    let out = Command::new(path)
        .spawn()
        .expect("failed to create python venv");

    println!("Output: {:?}", out.stdout);
}


pub fn run_script(script_path: &str, args: Vec<String>) -> ExitStatus {

    let activate_script = if cfg!(windows) {
        format!("{}\\Scripts\\activate.bat", VENV_DIR)
    } else {
        format!("{}\\bin\\activate", VENV_DIR)
    };

    let mut values = Vec::new();

    for arg in args {
        values.push(arg);
    }

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

    return child.wait().expect("failed to wait on child");
}