use std::env;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    //can use for each with a closure
    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{line}");
    }

    Ok(())
}

// creating search and search insensitive mught have been smarter. Line should also be transformed
// into lower case.
pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    let formatted_query: String = match ignore_case {
        true => query.to_lowercase(),
        false => query.to_string(),
    };

    for line in contents.lines() {
        if line.contains(&formatted_query) {
            result.push(line)
        };
    }
    result
}

pub struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}

impl Config {
    //Constructor for the config
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}
