use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::shell::alias::AliasManager;

pub struct Job {
    id: usize,
    command: String,
    process: Child,
}

impl Job {
    fn new(id: usize, command: String, process: Child) -> Self {
        Job { id, command, process }
    }

    pub fn display(&self) {
        println!("[Job ID: {}] Command: {}", self.id, self.command);
    }
}

pub struct JobManager {
    jobs: Arc<Mutex<HashMap<usize, Job>>>,
    next_job_id: usize,
}

impl JobManager {
    pub fn new() -> Self {
        JobManager {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            next_job_id: 1,
        }
    }

    pub fn start_background_job(&mut self, command: &str, aliases: &AliasManager) {
        let (cmd, args) = parse_command(command, aliases);
        let process = match Command::new(&cmd).args(&args).spawn() {
            Ok(child) => child,
            Err(e) => {
                eprintln!("Failed to start job '{}': {}", command, e);
                return;
            }
        };

        let job_id = self.next_job_id;
        self.next_job_id += 1;

        let job = Job::new(job_id, command.to_string(), process);

        {
            let mut jobs_guard = self.jobs.lock().unwrap();
            jobs_guard.insert(job_id, job);
        }

        let jobs = Arc::clone(&self.jobs);
        thread::spawn(move || {
            let mut jobs_guard = jobs.lock().unwrap();
            if let Some(job) = jobs_guard.get_mut(&job_id) {
                if let Err(e) = job.process.wait() {
                    eprintln!("Failed to wait on job {}: {}", job_id, e);
                }
                jobs_guard.remove(&job_id);
            }
        });

        println!("[{}] {}", job_id, command);
    }

    pub fn list_jobs(&self) {
        let jobs_guard = self.jobs.lock().unwrap();
        for job in jobs_guard.values() {
            job.display();
        }
    }

    pub fn foreground(&self, input: &str) {
        let job_id = input.split_whitespace().nth(1).and_then(|s| s.parse::<usize>().ok());
        if let Some(job_id) = job_id {
            let mut jobs_guard = self.jobs.lock().unwrap();
            if let Some(mut job) = jobs_guard.remove(&job_id) {
                if let Err(e) = job.process.wait() {
                    eprintln!("Failed to wait on job {}: {}", job_id, e);
                }
            } else {
                eprintln!("fg: job not found: {}", job_id);
            }
        } else {
            eprintln!("Usage: fg <job_id>");
        }
    }

    pub fn background(&self, input: &str) {
        let job_id = input.split_whitespace().nth(1).and_then(|s| s.parse::<usize>().ok());
        if let Some(job_id) = job_id {
            let jobs_guard = self.jobs.lock().unwrap();
            if let Some(job) = jobs_guard.get(&job_id) {
                println!("[{}] {}", job_id, job.command);
            } else {
                eprintln!("bg: job not found: {}", job_id);
            }
        } else {
            eprintln!("Usage: bg <job_id>");
        }
    }
}

fn parse_command(input: &str, aliases: &AliasManager) -> (String, Vec<String>) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = aliases.expand_alias(parts[0]);
    let args = parts[1..].iter().map(|&s| s.to_string()).collect();
    (command.to_string(), args)
}
