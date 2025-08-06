use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case;
        // here i am giving priority to arguments passed and then to environment variable
        if args.len() == 4 && !args[3].is_empty() {
            match args[3].chars().next().unwrap().to_ascii_lowercase() {
                't' => ignore_case = true,
                'f' => ignore_case = false,
                _ => ignore_case = env::var("IGNORE_CASE").is_ok(),
            }
        } else {
            ignore_case = env::var("IGNORE_CASE").is_ok();
        }


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
    let mut results: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
} 

fn search_case_insensitive<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

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