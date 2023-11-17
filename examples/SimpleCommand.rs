extern crate rusty_cmd;

use rusty_cmd::*;

struct SimpleCommand;
impl CommandHandler for SimpleCommand {
    fn execute(&self, _line: Option<String>) {
        println!("Hello, this is Simple!");
    }

    fn get_help_string(&self) -> String {
        String::from("Greeting from Simple")
    }
}

//  rustc SimpleCommand.rs --extern rusty_cmd=../target/debug/librusty_cmd.rlib
fn main() {
    let mut cmd: rusty_cmd::Cmdline = rusty_cmd::Cmdline::new();
    cmd.change_prompt("[Simple]# ");
    cmd.add_command("simple", Box::new(SimpleCommand));
    let _ = cmd.cmdloop();
}