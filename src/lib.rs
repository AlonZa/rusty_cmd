use std::collections::HashMap;
use std::io::{self, Write};
use std::process::exit;

const DEFAULT_PROMPT: &str = "rusty_cmd $ ";

pub trait CommandHandler {
    fn execute(&self, line: Option<String>);
    fn get_help_string<'a>(&self) -> &'a str {
        "No help here..."
    }
}

pub struct Cmdline<'a> {
    prompt: &'a str,
    commands: HashMap<&'a str, Box<dyn CommandHandler>>,
}

impl<'a> Cmdline<'a> {
    pub fn new() -> Cmdline<'a> {
        let mut cmdline = Cmdline {
            prompt: DEFAULT_PROMPT,
            commands: HashMap::new(),
        };
        cmdline.add_command("quit", Box::new(DefaultQuitCommand));
        cmdline
    }

    /// Run the main loop of the cmd-like program
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut cmd: rusty_cmd::Cmdline = rusty_cmd::Cmdline::new();
    /// cmd.cmdloop();
    /// ```
    pub fn cmdloop(&self) -> Result<(), io::Error> {

        loop {
            print!("{}", self.prompt);
            io::stdout().flush()?;

            let mut command_line: String = String::new();

            io::stdin().read_line(&mut command_line)?;

            let command_line: &str = command_line.trim();
            if command_line.is_empty() {
                println!("");
                continue;
            }

            let (cmd, line) = self.parse_command(&command_line);
            if let Some(handler) = self.commands.get(cmd) {
                handler.execute(line);
            }
            else {
                if cmd.eq("help") {
                    self.help_command();
                    continue;
                }
                println!("Unknown command: {}", cmd);
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

    pub fn get_prompt(&self) -> &'a str {
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

    fn help_command(&self) {
        if self.commands.get("help").is_none() {
            println!("help:\n\tThis help menu")
        }
        for command in &self.commands {
            print!("{}:\n", command.0);
            let lines = command.1.get_help_string();
            let lines = lines.split('\n');
            for line in lines {
                println!("\t{}", line);
            }
        }
    }

}

struct DefaultQuitCommand;
impl CommandHandler for DefaultQuitCommand {
    fn execute(&self, _line: Option<String>) {
        println!("Quitting...");
        exit(0);
    }

    fn get_help_string<'a>(&self) -> &'a str {
        "Quit with exit code 0"
    }
}