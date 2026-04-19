use std::env;
use std::fs;
use std::process;

use rbasic::{interpreter, parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <fichier.bas>", args[0]);
        process::exit(1);
    }

    let path = &args[1];
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Erreur lecture '{}': {}", path, e);
            process::exit(1);
        }
    };

    match parser::parse(source.trim()) {
        Ok(program) => {
            interpreter::run(&program);
        }
        Err(errors) => {
            for e in errors {
                eprintln!("Erreur de parsing: {:?}", e);
            }
            process::exit(1);
        }
    }
}
