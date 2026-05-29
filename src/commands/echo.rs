use crate::shell::Command;

///Echo func simply takes whatever the user types into their terminal
///Does processing on the single quotes and returns the output to stdout
pub fn echo_func(command: &Command) {
    if command.args.is_empty() {
        println!("Please provide some text for grep");
        return;
    }
    println!("{}", &command.args.join(" "));
}
