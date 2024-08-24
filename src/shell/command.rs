use std::process::{Command, Stdio};
use crate::shell::alias::AliasManager;
use crate::shell::builtins::change_directory;
use crate::shell::theme::Theme;

pub fn execute_chain(input: &str, aliases: &AliasManager) {
    let commands: Vec<&str> = input.split("&&").collect();
    for command in commands {
        let trimmed_command = command.trim();
        let (cmd, args) = parse_command(trimmed_command, aliases);
        let status = execute_command(&cmd, &args, false);
        if !status {
            break;
        }
    }
}

pub fn execute_pipe(input: &str, aliases: &AliasManager) {
    let commands: Vec<&str> = input.split('|').collect();
    let mut previous_output = None;

    for (i, command) in commands.iter().enumerate() {
        let trimmed_command = command.trim();
        let (cmd, args) = parse_command(trimmed_command, aliases);

        let mut process = Command::new(&cmd);
        process.args(&args);

        if let Some(output) = previous_output.take() {
            process.stdin(Stdio::from(output));
        }

        if i < commands.len() - 1 {
            process.stdout(Stdio::piped());
        }

        let mut child = match process.spawn() {
            Ok(child) => child,
            Err(e) => {
                eprintln!("{}", Theme::error_text(&format!("Failed to spawn command '{}': {}", cmd, e)));
                return;
            }
        };

        previous_output = if i < commands.len() - 1 {
            child.stdout.take()
        } else {
            None
        };

        let status = match child.wait() {
            Ok(status) => status,
            Err(e) => {
                eprintln!("{}", Theme::error_text(&format!("Failed to wait on child process '{}': {}", cmd, e)));
                return;
            }
        };

        if !status.success() {
            break;
        }
    }
}

pub fn execute_single_command(input: &str, background: bool, aliases: &AliasManager) {
    let (command, args) = parse_command(input, aliases);
    match command.as_str() {
        "exit" => std::process::exit(0),
        "cd" => change_directory(&args),
        _ => {
            execute_command(&command, &args, background);
        }
    }
}

fn execute_command(command: &str, args: &[String], background: bool) -> bool {
    let mut cmd = Command::new(command);
    cmd.args(args);

    if background {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        match cmd.spawn() {
            Ok(child) => {
                println!("Started background job with PID: {}", child.id());
                true
            }
            Err(e) => {
                eprintln!("{}", Theme::error_text(&format!("Failed to start background job: {}", e)));
                false
            }
        }
    } else {
        match cmd.status() {
            Ok(status) => status.success(),
            Err(e) => {
                eprintln!("{}", Theme::error_text(&format!("Failed to execute command '{}': {}", command, e)));
                false
            }
        }
    }
}

fn parse_command(input: &str, aliases: &AliasManager) -> (String, Vec<String>) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = aliases.expand_alias(parts[0]);
    let args = parts[1..].iter().map(|&s| s.to_string()).collect();
    (command.to_string(), args)
}

pub fn execute_file(file_path: &str, args: &[String]) {
    let mut command = Command::new(file_path);
    command.args(args);

    match command.spawn() {
        Ok(mut child) => {
            let status = child.wait().expect("Failed to wait on child process");
            println!("{}", Theme::output_text(&format!("Process exited with status: {}", status)));
        }
        Err(e) => {
            eprintln!("{}", Theme::error_text(&format!("Failed to execute file '{}': {}", file_path, e)));
        }
    }
}