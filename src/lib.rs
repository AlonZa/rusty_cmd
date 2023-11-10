use std::collections::HashMap;
use std::io::{self, Write};

const DEFAULT_PROMPT: &str = "rusty_cmd $ ";

pub trait CommandHandler {
    fn execute(&self, line: Option<String>);
}

pub struct Cmdline {
    prompt: String,
    commands: HashMap<String, Box<dyn CommandHandler>>
}

impl Cmdline {
    pub fn new() -> Cmdline {
        Cmdline {
            prompt: String::from(DEFAULT_PROMPT),
            commands: HashMap::new(),
        }
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
                continue;
            }
            
            let (cmd, line) = self.parse_command(&command_line);
            let cmd = cmd.to_string();
            if let Some(handler) = self.commands.get(&cmd) {
                handler.execute(line);
            }
            else {
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
    pub fn change_prompt(&mut self, new_prompt: &str) {
        self.prompt = String::from(new_prompt);
    }

    pub fn get_prompt(&self) -> String {
        self.prompt.clone()
    }
    
    fn parse_command(&self, command_line: &str) -> (String, Option<String>) {
        let mut tokens = command_line.split_whitespace();
        let cmd: String = tokens.next().unwrap_or("").to_string();
        let line: String = tokens.collect::<Vec<_>>().join(" ");
        let line: Option<String> = if line.is_empty() {
            None
        } else {
            Some(line)
        };
        (cmd, line)
    }

    pub fn add_command(&mut self, name: &str, handler: Box<dyn CommandHandler>) {
        self.commands.insert(name.to_string(), handler);
    }

}

