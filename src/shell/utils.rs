use crate::shell::{Command, PATH};
use is_executable::IsExecutable;
use std::env::{self, VarError};
use std::path::Path;
use std::process::Command as TermCommand;

//TODO: this is janky, I have to send the split command to process_single_quotes
//Then I have to resplit the command again... Gotta do better then this
pub fn execute_binary(command: &Command) {
    let status = TermCommand::new(&command.name).args(&command.args).status();
    if status.is_err() {
        eprintln!("Failed.");
    }
}

//NOTE: a good optimization would be to just get this at app start and store in a map
//instead of checking each time

///This method will check if a binary exists for the command given
///command: &str --> full string for the terminal command given
///This will support type <command>
pub fn executable_path_exists(command: &Command) -> bool {
    let executables: Vec<String> = get_all_executables();
    if executables.is_empty() {
        panic!("unable to retrieve $PATH");
    }

    for nxt_path in &executables {
        let path_ = Path::new(nxt_path).to_owned();
        let joined_path = path_.join(&command.name);
        let exists = joined_path.is_executable();
        if exists {
            return true;
        }
    }
    false
}

pub fn execute_command(command: &Command) {
    if !executable_path_exists(command) {
        println!("{}: command not found", command.name);
        return;
    }
    execute_binary(command);
}

pub fn get_all_executables() -> Vec<String> {
    let full_path = _get_path().to_owned();
    match full_path {
        Ok(full_path) => full_path,
        Err(_) => Vec::new(),
    }
}

///This method will get the $PATH and return a Vector of strings for
///each item in path. The expectation here is that this method will just return the vector
///of paths and the caller will own the result. I think that's more idiomatic and flexible
pub fn _get_path() -> Result<Vec<String>, VarError> {
    let full_path = env::var(PATH)?;
    Ok(full_path.split(":").map(String::from).collect())
}

//Simple helper for printing that command is help method
//Using this to make main.rs more readable
pub fn _builtin(cmd: &str) {
    println!("{} is a shell builtin", cmd)
}
