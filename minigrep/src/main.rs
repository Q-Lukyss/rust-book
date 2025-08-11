
use std::env;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive, Config, run};
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            eprintln!("Probleme en parsant les arguments: {err}");
            process::exit(1);
        }
    );
    
    if let Err(e) = run(config) {
        eprintln!("Erreur de l'Application: {e}");
        process::exit(1);
    }
}