use std::env;
use std::fs;
use std::process;

struct Config {
    show_all: bool,
    path: String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut show_all = false;
        let mut path = String::from(".");

        let mut i = 1; // Skip program name
        while i < args.len() {
            match args[i].as_str() {
                "-a" | "--all" => show_all = true,
                arg if arg.starts_with('-') => {
                    return Err("Invalid argument");
                }
                _ => {
                    if path == "." {
                        path = args[i].clone();
                    } else {
                        return Err("Too many arguments");
                    }
                }
            }
            i += 1;
        }

        Ok(Config { show_all, path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match fs::read_dir(&config.path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        if let Some(file_name) = e.file_name().to_str() {
                            // Filter hidden files unless show_all is true
                            if config.show_all || !file_name.starts_with('.') {
                                println!("{}", file_name);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory '{}': {}", &config.path, e),
    }
}
