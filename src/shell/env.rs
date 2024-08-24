use std::env;

pub fn set_env_var(args: &[String]) {
    if args.len() == 2 {
        env::set_var(&args[0], &args[1]);
    } else {
        eprintln!("Usage: set VAR value");
    }
}

pub fn unset_env_var(args: &[String]) {
    if args.len() == 1 {
        env::remove_var(&args[0]);
    } else {
        eprintln!("Usage: unset VAR");
    }
}

pub fn print_env_vars() {
    let vars: Vec<(String, String)> = env::vars().collect();
    if vars.is_empty() {
        println!("No environment variables are set.");
    } else {
        for (key, value) in vars {
            println!("{}={}", key, value);
        }
    }
}
