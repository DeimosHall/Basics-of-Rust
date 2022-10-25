// fs is to manage file system
use std::env;
use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /*
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    } */

    // Result<Ok, Err>
    // args can be any time that implements the Iterator trait
    // and returns String items
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        /*
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 3 {
            return Err("too many arguments");
        } */

        args.next(); // We ignore the name of the program here

        let query = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get any query string"),
        };

        let file_path = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get any file path"),
        };

        /*
         * export IGNORE_CASE=1 -> It will create the eviroment
         * variable that will be available only in that shell
         * session
         */
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("IGNORE_CASE: {}", ignore_case);

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/* With this lifetime annotation we tell Rust that the date returned
 * by the function will live as long as the data passed into the function
 * in the contents argument.
 */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
    results.push(line);
    }
    }

    results */

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct"; // productive contains duct
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
