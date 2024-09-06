use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Please recheck your args.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&cfg.file_path)?;
    let res = search(&cfg.query, &content);
    dbg!(res);
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
