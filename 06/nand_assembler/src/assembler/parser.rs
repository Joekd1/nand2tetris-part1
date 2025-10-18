use std::error::Error;
use std::fs;

pub struct Parser {
    pub contents: Vec<String>,
    pub file_index: i32, 
}


impl Parser {
    pub fn build(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents: String = fs::read_to_string(file_path)?;

        let contents: Vec<_> = contents
            .lines()
            .map(|s| s.trim().to_string())
            .collect();

	let file_index = -1;

        Ok(Self {
            contents,
	    file_index
        })
    }

    pub fn has_more_lines(&self) -> bool {
        // checks if there are still lines
        self.contents.len() as i32 > (self.file_index + 1)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    struct TestContext {
        empty: Parser,
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            println!("Test teardown ...");
        }
    }

    fn setup() -> TestContext {
        println!("Test setup ...");

        let contents = Vec::new();
        let file_index = -1;

        TestContext {
            empty: Parser {
                contents,
                file_index,
            }
	}
    }

    #[test]
    fn has_more_lines_of_empty_vec_is_false() {
        // Arrange
        let ctx = setup();

        // Assert
        assert_eq!(ctx.empty.has_more_lines(), false);
    }

}
