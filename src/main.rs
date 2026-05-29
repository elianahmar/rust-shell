//NOTE:
mod commands;
mod shell;

use commands::*;
use shell::*;

use std::io::{self, Write};

//NOTE: Important Note:
// In the terminal world their are four primary categories for commands
//  - single target commands -> except and operate on a single argument. Ex. python <file>.py
//  - multi target commands -> except multiple arguments. Ex. ls <dir1> <dir2> ... rm <dir1> <dir2>
//  - stream oriented commands -> expect streams (can allow multiple targets). "grep foo <file1> <file2>"
//  - orchestrator commands -> allow subcommands. Basically all CLI tools (kubectl, git, etc)

//TODO: need to handle empty enter
// - need to handle "type " case
// - ls fails. Due to bug where we call full_command_split[1]
// - Fix the processing of single quotes
// - refactor code base. Try to make methods small. Have one method do one thing
// - We execute binaries based on the command and it's arguments
// - Let's make the command an object. And pass that object around. That will simplify things
//
//
//DONE:
// - turn all print statements into some helper function
// - move the consts to shell package and import them
// - move all commands to their own files in a single module
//

fn main() {
    //TODO: here we need to collect all of the executables here and just reference this map
    //That will simplify the entire codebase

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut full_command = String::new();

        io::stdin().read_line(&mut full_command).unwrap();

        if full_command.is_empty() {
            continue;
        }

        let command: Command = get_command(&full_command);

        //TODO: I think here I refactor into seperate method or call a preprocess method here
        match command.name.as_str() {
            EXIT => break,
            ECHO => echo_func(&command),
            // TODO: refactor this as well into type func
            TYPE => type_func(&command),
            //TODO: ugly... clean this up
            PWD => {
                let _ = pwd_func();
            }
            CD => cd_func(&command),
            _ => execute_command(&command),
        };
    }
}
