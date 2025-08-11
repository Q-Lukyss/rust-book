use std::env;
use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // the dyn keyword is short for dynamic.
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
    pub fn build (args: &[String]) -> Result<Config, &'static str> {
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        match args.len() {
            3 | 4 => {
                let query = args[1].clone();
                let file_path = args[2].clone();
                if args.len() == 4 && (args[3] == "-i" || args[3] == "--insensitive"){
                    ignore_case = true;
                }
                Ok( Config {
                    query,
                    file_path,
                    ignore_case
                })
            }
            n if n < 3 => return Err("Pas assez d'arguments."),
            _ => return Err("Trop d'arguments."),
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let result = contents
        .lines()
        .filter(|x| x.contains(query))
        .collect();

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let result = contents
        .lines()
        .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
        .collect();

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}