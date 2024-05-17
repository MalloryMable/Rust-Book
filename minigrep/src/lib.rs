use std::env::var;
use std::fs::read_to_string;
use std::error::Error;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        match args.len() {
            1 => return Err("No arguments passed"),
            2 => return Err("No target file"),
            3 => {},
            _ => return Err("Too many arguments"),
        };

		    // NOTE: Find a non-clone solution
		    let query = args[1].clone();
		    let file_path = args[2].clone();

        let ignore_case = var("IGNORE_CASE").is_ok();

		   Ok(Config { query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {


    let file_content = read_to_string(config.file_path)?;
       // .expect("Should have been able to read from file");

    let results = if config.ignore_case{
        search_insensitive(&config.query, &file_content)
    } else {
        search_sensitive(&config.query, &file_content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}


fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

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
    fn one_result() {
        let query = "duct";
        let contents = "\\\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search_sensitive(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\\\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        
        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents))

    }
}


