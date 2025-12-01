use serde::{Serialize, de};
use tracing_subscriber::field::debug;

use crate::views::terminal_v::render_terminal_help;

#[derive(Debug, PartialEq)]
pub enum Command {
    Echo,
    Clear,
    Help,
    Empty,
    Unknown(String),
}

impl From<String> for Command {
    fn from(text: String) -> Self {
        match text.as_str() {
            "echo" => Command::Echo,
            "clear" | "cls" => Command::Clear,
            "help" => Command::Help,
            "" => Command::Empty,
            _ => Command::Unknown(text),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CommandLineOutput {
    pub output: String,
    pub script: String,
}

impl Default for CommandLineOutput {
    fn default() -> Self {
        Self {
            output: "".to_string(),
            script: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CommandLine {
    pub command: Command,
    pub args: Vec<String>,
}

impl Default for CommandLine {
    fn default() -> Self {
        Self {
            command: Command::Unknown(String::new()),
            args: Vec::new(),
        }
    }
}

impl From<&str> for CommandLine {
    fn from(text: &str) -> Self {
        let split_parts: Vec<String> = text.split_whitespace().map(|v| v.to_string()).collect();

        let mut command_line = CommandLine::default();

        if text.trim() == "" {
            command_line.command = Command::Empty;
        }

        if split_parts.len() > 0 {
            command_line.command = Command::from(split_parts[0].to_string());
        }

        if split_parts.len() > 1 {
            command_line.args = split_parts[1..].to_vec();
        }

        command_line
    }
}

impl CommandLine {
    pub fn execute(&self) -> CommandLineOutput {
        match &self.command {
            Command::Echo => CommandLineOutput {
                output: format!("{}", self.args.join(" ")),
                script: "".to_string(),
            },
            Command::Clear => CommandLineOutput::default(),
            Command::Help => CommandLineOutput {
                output: render_terminal_help(),
                script: "".to_string(),
            },
            Command::Empty => CommandLineOutput::default(),
            Command::Unknown(command) => CommandLineOutput {
                output: format!("Unknown command: {}", command),
                script: "".to_string(),
            },
        }
    }
}
