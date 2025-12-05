use deadpool_postgres::Pool;
use serde::Serialize;

use crate::{
    models::state::SessionMap,
    utilities::{terminal_cd::Cd, terminal_ls::Ls},
    views::terminal_v::render_terminal_help,
};

#[derive(Debug, PartialEq)]
pub enum Command {
    Echo,
    Clear,
    Help,
    Empty,
    Pwd,
    Cd,
    Ls,
    Unknown(String),
}

impl From<String> for Command {
    fn from(text: String) -> Self {
        match text.as_str() {
            "echo" => Command::Echo,
            "clear" | "cls" => Command::Clear,
            "help" => Command::Help,
            "pwd" => Command::Pwd,
            "cd" => Command::Cd,
            "ls" => Command::Ls,
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
pub struct CommandLine<'a> {
    pub command: Command,
    pub args: Vec<String>,
    pub session_map: &'a SessionMap,
    pub user_id: i32,
    pub pool: &'a Pool,
}

impl<'a> CommandLine<'a> {
    pub fn setup_command(
        text: &str,
        user_id: i32,
        session_map: &'a SessionMap,
        pool: &'a Pool,
    ) -> Self {
        let split_parts: Vec<String> = text.split_whitespace().map(|v| v.to_string()).collect();

        let mut command_line = CommandLine {
            command: Command::Unknown(String::new()),
            args: Vec::new(),
            user_id,
            session_map,
            pool,
        };

        if text.trim() == "" {
            command_line.command = Command::Empty;
        }

        if !split_parts.is_empty() {
            command_line.command = Command::from(split_parts[0].to_string());
        }

        if split_parts.len() > 1 {
            command_line.args = split_parts[1..].to_vec();
        }

        command_line
    }

    pub fn process_command(
        &self,
        output: Option<String>,
        script: Option<String>,
    ) -> CommandLineOutput {
        CommandLineOutput {
            output: output.unwrap_or_default(),
            script: script.unwrap_or_default(),
        }
    }

    pub async fn execute(&self) -> CommandLineOutput {
        match &self.command {
            Command::Echo => self.process_command(Some(self.args.join(" ").to_string()), None),
            Command::Help => self.process_command(Some(render_terminal_help()), None),
            Command::Pwd => {
                let session_map = self.session_map.pin_owned();
                let current_dir = session_map
                    .get(&format!("current-dir-{}", self.user_id))
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                self.process_command(Some(current_dir), None)
            }
            Command::Cd => {
                let session_map = self.session_map.pin_owned();
                let current_dir = session_map
                    .get(&format!("current-dir-{}", self.user_id))
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                let cd = Cd::new(&current_dir, &self.args, self.user_id, self.pool);
                let result = cd.go_to_path().await;

                match result {
                    Ok(path) => {
                        session_map.insert(format!("current-dir-{}", self.user_id), path.clone());

                        self.process_command(
                            Some("".into()),
                            Some(format!(
                                r#"
                                    <script type="module">
                                        import {{ replacePath }} from "/assets/js/terminal.js";
                                        replacePath("{}");
                                    </script>
                                "#,
                                path
                            )),
                        )
                    }
                    Err(error) => self.process_command(Some(error), None),
                }
            }
            Command::Ls => {
                let session_map = self.session_map.pin_owned();
                let current_dir = session_map
                    .get(&format!("current-dir-{}", self.user_id))
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                let ls = Ls::new(&current_dir, self.user_id, self.pool);
                let output = ls.list_file().await;
                self.process_command(Some(output), None)
            }
            Command::Clear => CommandLineOutput::default(),
            Command::Empty => CommandLineOutput::default(),
            Command::Unknown(command) => CommandLineOutput {
                output: format!(
                    "Unknown command: {} 💥💥💥. Type help more information 😚😚😚.",
                    command
                ),
                script: "".to_string(),
            },
        }
    }
}
