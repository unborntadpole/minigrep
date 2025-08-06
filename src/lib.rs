use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        args.next();

        let query = if let Some(str) = args.next() {str} else {
            return Err("No query string given");
        };

        let file_path = match args.next() {
            Some(str) => str,
            None => return Err("No file path given"),
        };

        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        let ignore_case = match args.next() {
            Some(str) if !str.is_empty() => {
                match str.chars().next().unwrap().to_ascii_lowercase() {
                    't' => true,
                    'f' => false,
                    _ => ignore_case_env,
                }
            }
            _ => ignore_case_env
        };

        Ok(
            Config{
                query,
                file_path,
                ignore_case,
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let result;
    if config.ignore_case {
        result = search_case_insensitive(&config.query, &contents);
    } else {
        result = search(&config.query, &contents);
    }
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let results: Vec<&'a str> = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
    results
} 

fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let results = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect();
    results
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