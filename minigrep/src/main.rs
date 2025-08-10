
use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("Probleme en parsant les arguments: {err}");
            process::exit(1);
        }
    );

    if let Err(e) = run(config) {
        println!("Erreur de l'Application: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> { // the dyn keyword is short for dynamic.
    let content = fs::read_to_string(config.file_path)?; // ? will return the error value from the current function for the caller to handle.

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Pas assez d'arguments.")
        }

        // On commence a index 1 car 0 est le chemin du binaire
        let query = &args[1].clone();
        let file_path = &args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query : query.to_string(),
            file_path : file_path.to_string(),
            ignore_case,
        })
    }
}