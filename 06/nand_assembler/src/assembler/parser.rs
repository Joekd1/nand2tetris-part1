use std::error::Error;
use std::fs;

pub struct Parser {
    pub contents: Vec<String>,
}


impl Parser {
    pub fn build(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents: String = fs::read_to_string(file_path)?;

        let contents: Vec<_> = contents
            .lines()
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Self {
            contents,
        })
    }
}
