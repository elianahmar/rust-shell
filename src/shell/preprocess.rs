use crate::shell::HOME;
///This method will fetch the command by itself
///We will not be returning the arguements passed to the
///command.
///Ex. Full Command = go [...]
///Output = go
//NOTE: according to POSIX-style vocab this is the correct terminology
//  - Name: this is the tool/utility we are using
//  - Options: this are the dashed args. For ex. "-r" when we do "rm -r <dir>"
//  - Operand: this is the target. We can have zero or many. Ex. cat <file1> <file2>
//  - Invocation: entire string input for the terminal
use std::env;

//
#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub home: String,
}

///This method creates a command object which stores all the relevant
///information for the input on the command given by the user
///NOTE: chatGPT bailed me out here...
///Basically for handling single quotes I couldn't get around tokenizing the inputs
///I'm iterating through each char, if I hit a single quote then I flip a flag
///Otherwise, I check for whitespace, and not in sq
pub fn get_command(command: &str) -> Command {
    let tokens = tokenize_quotes(command.trim());
    let name = tokens.get(0).cloned().unwrap_or_default();
    let args = if tokens.len() > 1 {
        tokens[1..].to_vec()
    } else {
        vec![]
    };

    Command {
        name,
        args,
        home: env::var(HOME).expect("Unable to access home path"),
    }
}
fn tokenize_quotes(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut cur = String::new();
    let mut in_sq = false;
    let mut in_dq = false;

    for ch in input.chars() {
        match ch {
            //NOTE: we are supporting for cases
            //If single quote, then we flip the flag if we are not in double quotes
            //If double quote, then we flip the flag if we are not in single quotes
            //if c is whitespace and we are not in sq and not in double quote
            '\'' => {
                if !in_dq {
                    in_sq = !in_sq
                } else {
                    cur.push(ch);
                }
            }
            '\"' => {
                if !in_sq {
                    in_dq = !in_dq
                } else {
                    cur.push(ch);
                }
            } // toggle, don't include quote
            c if c.is_whitespace() && !in_sq && !in_dq => {
                if !cur.is_empty() {
                    args.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(ch),
        }
    }

    if !cur.is_empty() {
        args.push(cur);
    }

    args
}
