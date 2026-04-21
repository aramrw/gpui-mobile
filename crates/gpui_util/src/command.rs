use std::process::Command;

pub fn new_command(program: &str) -> Command {
    Command::new(program)
}

pub fn new_std_command(program: &str) -> Command {
    Command::new(program)
}
