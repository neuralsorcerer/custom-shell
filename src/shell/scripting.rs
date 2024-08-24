use std::fs::read_to_string;
use crate::shell::command::execute_single_command;
use crate::shell::alias::AliasManager;

pub fn execute_script(script_path: &str, aliases: &mut AliasManager) {
    match read_to_string(script_path) {
        Ok(content) => {
            for line in content.lines() {
                let line = line.trim();
                if !line.is_empty() && !line.starts_with('#') {
                    execute_single_command(line, false, aliases);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read script file '{}': {}", script_path, e);
        }
    }
}
