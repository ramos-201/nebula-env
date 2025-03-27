use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{env, io};

pub const HISTORY_FILE: &str = ".nebula_history";

pub fn run_handler() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = args[1..].join(" ");
        let history_path = get_history_path().map_err(|e| e.to_string())?;
        store_command(&command, history_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn get_history_path() -> Result<PathBuf, env::VarError> {
    env::var("HOME").map(|home| {
        let mut path = PathBuf::from(home);
        path.push(HISTORY_FILE);
        path
    })
}

pub fn store_command(command: &str, history_path: PathBuf) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(history_path)?;
    writeln!(file, "{}", command)?;
    Ok(())
}
