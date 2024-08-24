use std::env;
use crate::shell::theme::Theme;

pub fn change_directory(args: &[String]) {
    let new_dir = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| {
            eprintln!("{}", Theme::error_text("cd: HOME environment variable not set"));
            return String::new();
        })
    } else {
        args[0].clone()
    };

    if !new_dir.is_empty() {
        if let Err(e) = env::set_current_dir(&new_dir) {
            eprintln!("{}", Theme::error_text(&format!("cd: {}: {}", e, new_dir)));
        }
    }
}

pub fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                println!("{}", Theme::directory_text(path_str));
            } else {
                eprintln!("{}", Theme::error_text("pwd: Failed to convert path to string"));
            }
        }
        Err(e) => eprintln!("{}", Theme::error_text(&format!("pwd: {}", e))),
    }
}
