use crate::shell::Command;
use std::env;
use std::path::{Path, PathBuf};

//TODO: need to fix "~" this case
pub fn cd_func(command: &Command) {
    let mut directory: PathBuf = PathBuf::from(&command.args.join(""));
    if command.args.len() == 1 && command.args[0] == "~" {
        directory = PathBuf::from(&command.home)
    }
    match env::set_current_dir(Path::new(&directory)) {
        Ok(()) => (),
        Err(_) => println!(
            "cd: {}: No such file or directory",
            directory.as_path().to_string_lossy()
        ),
    };
}
