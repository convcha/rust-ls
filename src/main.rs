use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };
    
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        if let Some(file_name) = e.file_name().to_str() {
                            println!("{}", file_name);
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory '{}': {}", path, e),
    }
}
