use std::collections::HashMap;
use std::io::{self, stdout};
use std::process::exit;

pub mod printerterm;
use printerterm::PrinterTerm;

const DEFAULT_PROMPT: &str = "rusty_cmd $ ";

pub trait CommandHandler {
    fn execute(&self, line: Option<String>, printer: &mut PrinterTerm);
    fn get_help_string<'a>(&self) -> &'a str {
        "No help here..."
    }
}

pub struct CmdLoop<'a> {
    printer: PrinterTerm,
    prompt: &'a str,
    commands: HashMap<&'a str, Box<dyn CommandHandler>>,
}

impl<'a> CmdLoop<'a> {
    pub fn new() -> CmdLoop<'a> {
        let mut cmd_loop = CmdLoop { 
            printer: PrinterTerm::new(),
            prompt: DEFAULT_PROMPT,
            commands: HashMap::new(),
        };
        cmd_loop.add_command("quit", Box::new(DefaultQuitCommand));
        cmd_loop
    }

        /// Run the main loop of the cmd-like program
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut cmd: rusty_cmd::Cmdline = rusty_cmd::Cmdline::new();
    /// cmd.cmdloop();
    /// ```
    pub fn cmdloop(&mut self) -> Result<(), io::Error> {

        self.printer.gracefully_start();
        loop {
            self.printer.print_wrapped(&format!("{}", self.prompt));

            let mut command_line: String = self.printer.read_input();

            let command_line: &str = command_line.trim();
            if command_line.is_empty() {
                println!("");
                continue;
            }
            
            let (cmd, line) = self.parse_command(&command_line);
            if let Some(handler) = self.commands.get(cmd) {
                handler.execute(line, &mut self.printer);
            }
            else {
                if cmd.eq("help") {
                    self.help_command();
                    continue;
                }
                self.printer.print_wrapped(&format!("Unknown command: {}", cmd));
            }
        }
    }

    /// Change the prompt of the cmd-like program
    ///
    /// # Arguments
    ///
    /// * new_prompt - A string slice that holds the new prompt.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cmd: rusty_cmd::Cmdline = rusty_cmd::Cmdline::new();
    /// cmd.change_prompt("[My New Prompt] # ");
    /// ```
    pub fn change_prompt(&mut self, new_prompt: &'a str) {
        self.prompt = new_prompt;
    }

    pub fn get_prompt(&self) -> &str {
        self.prompt
    }

    fn parse_command(&self, command_line: &'a str) -> (&'a str, Option<String>) {
        let mut tokens = command_line.split_whitespace();
        let cmd: &str = tokens.next().unwrap_or("");
        let line: String = tokens.collect::<Vec<_>>().join(" ");
        let line: Option<String> = if line.is_empty() {
            None
        } else {
            Some(line)
        };
        (cmd, line)
    }

    pub fn add_command(&mut self, name: &'a str, handler: Box<dyn CommandHandler>) {
        if self.commands.get(name).is_some() {
            self.commands.remove(name);
        }
        self.commands.insert(name, handler);
    }

    fn help_command(&mut self) {
        if self.commands.get("help").is_none() {
            self.printer.print_wrapped("help:\n\tThis help menu")
        }
        
        let commands = std::mem::take(&mut self.commands);
        for command in &commands {
            self.printer.print_wrapped(&format!("{}:\n", command.0));
            let lines = command.1.get_help_string();
            let lines = lines.split('\n');
            for line in lines {
                self.printer.print_wrapped(&format!("\t{}", line));
            }
        }
        self.printer.update_cursor(printerterm::Direction::Down(1));
        self.commands = commands;
    }
}


struct DefaultQuitCommand;
impl CommandHandler for DefaultQuitCommand {
    fn execute(&self, _line: Option<String>, cmd_printer: &mut PrinterTerm) {
        cmd_printer.print_wrapped("Quitting...");
        
        // Gracefully exit
        cmd_printer.gracefully_quit();
    }

    fn get_help_string<'a>(&self) -> &'a str {
        "Quit with exit code 0"
    }
}