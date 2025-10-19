use std::error::Error;
use std::fs;

pub struct Parser {
    pub contents: Vec<String>,
    pub file_index: i32,
    pub current_line: i32,
}

impl Parser {
    pub fn build(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents: String = fs::read_to_string(file_path)?;

        let contents: Vec<_> = contents.lines().map(|s| s.trim().to_string()).collect();

        let file_index = -1;
        let current_line = -1;

        Ok(Self {
            contents,
            file_index,
            current_line,
        })
    }

    pub fn has_more_lines(&self) -> bool {
        // checks if there are still lines
        self.contents.len() as i32 > (self.file_index + 1)
    }
}

    pub fn advance(&mut self) -> Result<(), Box<dyn Error>> {
        self.has_more_lines()
            .then(|| {
                self.file_index += 1;
                self.contents.get(self.file_index as usize)
            })
            .ok_or_else(|| Box::<dyn Error>::from("You reached the end of the file"))?
            .filter(|i| !i.is_empty() && !i.starts_with("//"))
            .map(|_| self.current_line += 1);

        Ok(())
    }

#[cfg(test)]
mod tests {

    use super::*;

    struct TestContext {
        empty: Parser,
        default: Parser,
        default_with_comments: Parser,
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
        let current_line = -1;

        TestContext {
            empty: Parser {
                contents,
                file_index,
                current_line,
            },

            default: Parser {
                contents: vec!["@A".to_string(), "@D".to_string()],
                current_line,
                file_index,
            },

            default_with_comments: Parser {
                contents: vec!["//Comment".to_string(), "@A".to_string(), "//Comment".to_string(), "@D".to_string()],
                current_line,
                file_index,
            },
        }
    }

    #[test]
    fn has_more_lines_of_empty_vec_is_false() {
        // Arrange
        let ctx = setup();

        // Assert
        assert_eq!(ctx.empty.has_more_lines(), false);
    }

    #[test]
    fn has_more_lines_non_empty_vec_initially_true() {
        // Arrange
        let ctx = setup();

        // Assert
        assert_eq!(ctx.default.has_more_lines(), true);
    }

    #[test]
    fn has_more_lines_at_end_of_vec_false() {
        // Apprange
        let mut ctx = setup();

        // Apply
        // lines start at 0 so 1 is the second and last line
        ctx.default.file_index = 1;

        //Assert
        assert_eq!(ctx.default.has_more_lines(), false);
    }

    #[test]
    fn advance_empty_contents_returns_error() {
        // Arrange
        let mut ctx = setup();
        let err = ctx.empty.advance().unwrap_err();

        //Assert
        assert_eq!(err.to_string(), "You reached the end of the file");
    }

    #[test]
    fn advance_increments_current_line() {
        // Arrange
        let mut ctx = setup();

        // Apply
        let current_line = match ctx.default.advance() {
            Ok(()) => ctx.default.current_line,
            _ => -1,
        };

        //Assert
        assert_eq!(current_line, 0);
    }

    #[test]
    fn advance_does_not_increments_current_line_for_comments() {
        // Arrange
        let mut ctx = setup();

        // Apply
        let current_line = match ctx.default_with_comments.advance() {
            Ok(()) => ctx.default.current_line,
            _ => -1,
        };

        //Assert
        assert_eq!(current_line, -1);
    }

    #[test]
    fn advance_end_of_file_returns_error() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.default.file_index = 1;
        let err = ctx.default.advance().unwrap_err();

        //Assert
        assert_eq!(err.to_string(), "You reached the end of the file");
    }
}
