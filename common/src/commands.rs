use inline_colorization::*;
use std::{env, process::ExitCode};

pub struct Command {
    name: &'static str,
    description: &'static str,
    usage: &'static str,
    run: fn(command_name: &str, args: &mut env::Args) -> Result<(), String>,
}

pub struct CommandExecutor {
    commands: Vec<Command>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn new_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    fn usage(&self, program: &str) {
        eprintln!("Usage: {program} <command>");
        eprintln!("Commands:");
        for Command {
            name,
            description,
            usage,
            ..
        } in self.commands.iter()
        {
            eprintln!("    {name} - {description}");
            eprintln!("        Usage: {name} {usage}");
        }
    }

    pub fn run(self) -> ExitCode {
        let mut args = env::args();
        let program = args.next().expect("No program args");
        if let Some(command_name) = args.next() {
            if let Some(command) = self
                .commands
                .iter()
                .find(|command| command.name == command_name)
            {
                match (command.run)(&command_name, &mut args) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(error) => {
                        eprintln!(
                            "{style_bold}{color_red}error{color_reset}: {error}{style_reset}"
                        );
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                self.usage(&program);
                eprintln!("{style_bold}{color_red}error{color_reset}: Unknown command {command_name}{style_reset}");
                return ExitCode::FAILURE;
            }
        } else {
            self.usage(&program);
            eprintln!("{style_bold}{color_red}error{color_reset}: No sub command is provided {style_reset}");
            return ExitCode::FAILURE;
        }
    }
}

impl Command {
    pub fn new(
        name: &'static str,
        description: &'static str,
        usage: &'static str,
        run: fn(command_name: &str, args: &mut env::Args) -> Result<(), String>,
    ) -> Self {
        Self {
            name,
            description,
            usage,
            run,
        }
    }
}
