pub mod builtins;
pub mod command;
pub mod env;
pub mod alias;
pub mod scripting;
pub mod job_control;
pub mod theme;

use builtins::{change_directory, print_working_directory};
use rustyline::Editor;
use command::*;
use env::*;
use job_control::*;
use scripting::execute_script;
use theme::Theme;
use crate::shell::alias::AliasManager;
use std::time::Instant;

pub struct Shell<'a> {
    history: Vec<String>,
    aliases: AliasManager<'a>,
    jobs: JobManager,
}

impl<'a> Shell<'a> {
    pub fn new() -> Self {
        println!("{}", Theme::welcome_message());
        Shell {
            history: Vec::new(),
            aliases: AliasManager::new(),
            jobs: JobManager::new(),
        }
    }

    pub fn run(&mut self) {
        let mut rl = Editor::<()>::new();
        loop {
            let current_dir = std::env::current_dir().unwrap();
            let prompt = format!(
                "{} {} ",
                Theme::directory_text(current_dir.to_str().unwrap()),
                Theme::prompt_text()
            );

            let readline = rl.readline(&prompt);
            match readline {
                Ok(line) => {
                    let input = line.trim();
                    if input.is_empty() {
                        continue;
                    }
                    rl.add_history_entry(input);
                    self.history.push(input.to_string());

                    let start_time = Instant::now();

                    let parts: Vec<&str> = input.split_whitespace().collect();
                    match parts.get(0).map(|&s| s) {
                        Some("set") => set_env_var(&parts[1..].iter().map(|s| s.to_string()).collect::<Vec<String>>()),
                        Some("unset") => unset_env_var(&parts[1..].iter().map(|s| s.to_string()).collect::<Vec<String>>()),
                        Some("env") => print_env_vars(),
                        Some("cd") => change_directory(&parts[1..].iter().map(|s| s.to_string()).collect::<Vec<String>>()),
                        Some("pwd") => print_working_directory(),
                        Some("jobs") => self.jobs.list_jobs(),
                        Some("fg") => self.jobs.foreground(input),
                        Some("bg") => self.jobs.background(input),
                        Some("alias") => {
                            if parts.len() == 3 {
                                self.aliases.add_alias(parts[1], parts[2]);
                                println!("{}", Theme::output_text(&format!("Alias added: {} -> {}", parts[1], parts[2])));
                            } else {
                                eprintln!("{}", Theme::error_text("Usage: alias name command"));
                            }
                        }
                        Some("unalias") => {
                            if parts.len() == 2 {
                                self.aliases.remove_alias(parts[1]);
                                println!("{}", Theme::output_text(&format!("Alias removed: {}", parts[1])));
                            } else {
                                eprintln!("{}", Theme::error_text("Usage: unalias name"));
                            }
                        }
                        Some("source") => {
                            if parts.len() == 2 {
                                execute_script(parts[1], &mut self.aliases);
                            } else {
                                eprintln!("{}", Theme::error_text("Usage: source scriptfile"));
                            }
                        }
                        _ => {
                            if std::path::Path::new(input).is_file() {
                                execute_file(input, &parts[1..].iter().map(|s| s.to_string()).collect::<Vec<String>>());
                            } else {
                                println!("{}", Theme::command_text(&format!("Executing command: {}", input)));
                                if input.contains("&&") || input.contains("||") {
                                    execute_chain(input, &self.aliases);
                                } else if input.contains("|") {
                                    execute_pipe(input, &self.aliases);
                                } else if input.ends_with('&') {
                                    self.jobs.start_background_job(input, &self.aliases);
                                } else {
                                    execute_single_command(input, false, &self.aliases);
                                }
                            }
                        }
                    }

                    let duration = start_time.elapsed();
                    println!("{}", Theme::execution_time_text(duration.as_secs_f64()));
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}
