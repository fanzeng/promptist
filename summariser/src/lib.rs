use std::error::Error;
use std::fs;
use std::io::Read;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let mut count = 0;
    let s = String::from("----------------"); // Separator
    println!("{} {} {}", s, String::from("Start"), s);
    for line in contents.lines() {
        println!("{line}");
        count += line.chars().count();
        if count > config.section_len.try_into().unwrap() {
            std::io::stdin().read(&mut [0u8]).unwrap();
            println!("{} {} {}", s, count, s);
            count = 0;
        }
    }
    Ok(())
}

pub struct Config {
    pub file_path: String,
    pub section_len: i32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let section_len: i32 = match args.next() {
            Some(arg) => arg.parse().expect("Error parsing section_len"),
            None => return Err("Didn't get a section length"),
        };
        Ok(Config {
            file_path,
            section_len,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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

    // #[test]
    //     fn case_sensitive() {
    //         let query = "duct";
    //         let contents = "\
    // Rust:
    // safe, fast, productive.
    // Pick three.
    // Duct tape.";
    //         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    //     }

    // #[test]
    //     fn case_insensitive() {
    //         let query = "rUsT";
    //         let contents = "\
    // Rust:
    // safe, fast, productive.
    // Pick three.
    // Trust me.";
    //         assert_eq!(
    //             vec!["Rust:", "Trust me."],
    //             search_case_insensitive(query, contents)
    //         );
    //     }
}
