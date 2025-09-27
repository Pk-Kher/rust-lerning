use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(data) => data,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(data) => data,
            None => return Err("Didn't get a file path string"),
        };
        // IGNORE_CASE=0 cargo run how poem.txt
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        // let ignore_case = match args.get(3) {
        //     Some(..) => true,
        //     None => false,
        // };
        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case: ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // if any library returning Result then you should use ?
    let content = fs::read_to_string(&config.file_path)?;
    let find_data = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for i in &find_data {
        println!("{}", i);
    }
    if find_data.len() == 0 {
        println!("Search value is not available");
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let data: Vec<&str> = content
        .lines()
        .filter(|v| !v.is_empty() && v.contains(&query))
        .collect();
    data
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let data: Vec<&str> = content
        .lines()
        .filter(|v| v.to_lowercase().contains(&query))
        .collect();
    data
}

// test cases
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_function() {
        let query = "safe";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
    #[test]
    fn case_insensitive_test() {
        let query = "Safe";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, content)
        )
    }
}
