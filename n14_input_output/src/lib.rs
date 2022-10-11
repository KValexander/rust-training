/* Use */
use std::error::Error;
use std::env;
use std::fs;

/* Struct Config */
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/* Impl Config */
impl Config {
    /* Parse arguments */
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        /* Check arguments */
        if args.len() < 3 {
            return Err("Not enought arguments");
        }

        /* Arguments */
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }    
}

/* Run */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /* Read file */
    let contents = fs::read_to_string(config.file_path)?;

    /* Search results */
    let results = if config.ignore_case {
        search_case_insensetive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    /* Search line */
    for line in results {
        println!("{line}\n");
    }

    Ok(())
}

/* Search line */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        } 
    }
    results
}

/* Search case insensetive */
pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        } 
    }
    results
}

/* Tests */
#[cfg(test)]
mod tests {
    use super::*;

    /* Case sensetive */
    #[test]
    fn case_sensetive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /* Case insensetive */
    #[test]
    fn case_insensetive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
        ";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensetive(query, contents));
    }

}