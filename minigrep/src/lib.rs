use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.get_file_path())?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {

        args.next(); // Skip the first argument which is the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Search String is missing!")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path is missing!")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
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
        let query = "shubh";
        let content = "\
Hello, world!
shubh
Hello, sam!";

        assert_eq!(vec!["shubh"], search(query, content));
    }

    #[test]
    fn case_insitive() {
        let query = "SHuBh";
        let content = "\
Hello, world!
shubh
Hello, sam!";

        assert_eq!(vec!["shubh"], search_case_insensitive(query, content));
    }
}