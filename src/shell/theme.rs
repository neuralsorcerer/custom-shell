use colored::*;

pub struct Theme;

impl Theme {
    pub fn prompt_text() -> ColoredString {
        "shell>".bright_blue().bold()
    }

    pub fn directory_text(dir: &str) -> ColoredString {
        dir.bright_cyan().bold()
    }

    pub fn error_text(message: &str) -> ColoredString {
        message.red().bold()
    }

    pub fn command_text(cmd: &str) -> ColoredString {
        cmd.green()
    }

    pub fn output_text(output: &str) -> ColoredString {
        output.white()
    }

    pub fn welcome_message() -> ColoredString {
        r#"
  ____  _          _ _ 
 / ___|| |__   ___| | |
 \___ \| '_ \ / _ \ | |
  ___) | | | |  __/ | |
 |____/|_| |_|\___|_|_|
                       
        Custom Shell
"#.blue().bold()
    }

    pub fn execution_time_text(time: f64) -> ColoredString {
        format!("Execution time: {:.3} seconds", time).yellow().italic()
    }
}
