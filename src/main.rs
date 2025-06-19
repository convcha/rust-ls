use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse arguments for flags and path
    let mut show_all = false;
    let mut path = ".";
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg == "-a" {
            show_all = true;
        } else if arg.starts_with('-') {
            eprintln!("Error: Unknown option '{}'", arg);
            eprintln!("Usage: {} [-a] [directory]", args[0]);
            eprintln!("  -a    show all files including hidden files");
            std::process::exit(1);
        } else {
            path = arg;
        }
        i += 1;
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        if let Some(file_name) = e.file_name().to_str() {
                            // Filter out hidden files unless -a flag is used
                            if show_all || !file_name.starts_with('.') {
                                println!("{}", file_name);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory '{}': {}", path, e),
    }
}
