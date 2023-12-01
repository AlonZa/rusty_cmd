use rusty_cmd;
use rusty_cmd::*;

struct SimpleCommand;
impl CommandHandler for SimpleCommand {
    fn execute(&self, _line: Option<String>, cmd_printer: &mut rusty_cmd::printerterm::PrinterTerm) {
        cmd_printer.print_wrapped("Greeting mate...");
    }

    fn get_help_string<'a>(&self) -> &'a str {
        "Get some greetings"
    }
}

//  rustc SimpleCommand.rs --extern rusty_cmd=../target/debug/librusty_cmd.rlib
fn main() {
    let mut cmd: rusty_cmd::CmdLoop = rusty_cmd::CmdLoop::new();
    cmd.change_prompt("[Simple]# ");
    cmd.add_command("simple", Box::new(SimpleCommand));
    let _ = cmd.cmdloop();
}
