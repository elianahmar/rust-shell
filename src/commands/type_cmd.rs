use crate::shell::*;
use is_executable::IsExecutable;
use std::path::Path;

pub fn type_func(command: &Command) {
    //Ensure we have a type to check.
    if command.args.is_empty() {
        return;
    }
    //NOTE: reason for iterating is because I can do something like
    //type go node python cat ls ... and get a response back for each of them
    for arg in command.args.iter() {
        match arg.as_str() {
            ECHO | EXIT | TYPE | PWD | CD => _builtin(&arg),
            _ => binary_exists(&command),
        }
    }
}

//TODO: has to be a cleaner way to do this??
//I think for now, we move this to type_cmd.rs. This does the same exact thing
//that executable_exists does. Gotta be a better way to do this
pub fn binary_exists(command: &Command) {
    //TODO: fetch the full path. Make that a method
    let executables: Vec<String> = get_all_executables();
    if executables.is_empty() {
        panic!("unable to retrieve $PATH");
    }

    for nxt_path in &executables {
        let path_ = Path::new(nxt_path).to_owned();
        let joined_path = path_.join(&command.args[0]);
        let exists = joined_path.is_executable();
        if exists {
            println!("{} is {}", &command.args[0], joined_path.to_string_lossy());
            return;
        }
    }
    println!("{}: not found", &command.args[0]);
}
