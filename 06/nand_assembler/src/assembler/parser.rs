use instruction::{Instruction, InstructionType};
use std::error::Error;
use std::fs;

mod instruction;

pub struct Parser {
    pub contents: Vec<String>,
    pub file_index: i32,
    pub current_line: i32,
    pub instruction: Instruction,
}

impl Parser {
    pub fn build(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let contents: String = fs::read_to_string(file_path)?;

        let contents: Vec<_> = contents.lines().map(|s| s.trim().to_string()).collect();

        let file_index = -1;
        let current_line = -1;
        let instruction = Instruction::new();

        Ok(Self {
            contents,
            file_index,
            current_line,
            instruction,
        })
    }

    pub fn has_more_lines(&self) -> bool {
        // checks if there are still lines
        self.contents.len() as i32 > (self.file_index + 1)
    }

    pub fn advance(&mut self) -> Result<(), Box<dyn Error>> {
        self.has_more_lines()
            .then(|| {
                self.file_index += 1;
                self.contents.get(self.file_index as usize)
            })
            .ok_or_else(|| Box::<dyn Error>::from("You reached the end of the file"))?
            .filter(|i| !i.is_empty() && !i.starts_with("//"))
            .map(|valid_instruction| {
                self.current_line += 1;
                self.instruction.current = Some(valid_instruction.clone())
            });

        Ok(())
    }

    pub fn instruction_type(&self) -> Option<InstructionType> {
        self.instruction.current.as_ref().and_then(|current| {
            if self.instruction.regex.reg_a.is_match(current) {
                Some(InstructionType::AInstruction)
            } else if self.instruction.regex.reg_c.is_match(current) {
                Some(InstructionType::CInstruction)
            } else if self.instruction.regex.reg_l.is_match(current) {
                Some(InstructionType::LInstruction)
            } else {
                None
            }
        })
    }

    pub fn symbol(&self) -> Option<&str> {
        self.instruction.current.as_ref().and_then(|current_inst| {
            let regex = match self.instruction_type() {
                Some(InstructionType::AInstruction) => &self.instruction.regex.reg_a,
                Some(InstructionType::LInstruction) => &self.instruction.regex.reg_l,
                _ => return None,
            };

            regex
                .captures(current_inst)
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str())
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct TestContext {
        empty: Parser,
        default: Parser,
        default_with_comments: Parser,
        a_inst_parser: Parser,
        c_inst_parser: Parser,
        l_inst_parser: Parser,
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
                instruction: Instruction::new(),
            },

            default: Parser {
                contents: vec!["@A".to_string(), "@D".to_string()],
                current_line,
                file_index,
                instruction: Instruction::new(),
            },

            default_with_comments: Parser {
                contents: vec![
                    "//Comment".to_string(),
                    "@A".to_string(),
                    "//Comment".to_string(),
                    "@D".to_string(),
                ],
                current_line,
                file_index,
                instruction: Instruction::new(),
            },

            a_inst_parser: Parser {
                contents: vec![
                    "@D".to_string(),
                    "@123".to_string(),
                    "@abc".to_string(),
                    "@y".to_string(),
                ],
                file_index,
                current_line,
                instruction: Instruction::new(),
            },

            c_inst_parser: Parser {
                contents: vec![
                    "D=M-D".to_string(),
                    "D=M".to_string(),
                    "0;JMP".to_string(),
                    "MD=D+M".to_string(),
                    "D;JNEQ".to_string(),
                ],
                file_index,
                current_line,
                instruction: Instruction::new(),
            },

            l_inst_parser: Parser {
                contents: vec![
                    "(123)".to_string(),
                    "(LOOP)".to_string(),
                    "(END)".to_string(),
                ],
                file_index,
                current_line,
                instruction: Instruction::new(),
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

    #[test]
    fn current_instruction_before_advance_is_none() {
        // Arrange
        let ctx = setup();

        // Assert
        assert_eq!(ctx.default.instruction.current, None);
    }

    #[test]
    fn current_instruction_before_valid_instruction_is_none() {
        // Arrange
        let mut ctx = setup();

        //Apply
        ctx.default_with_comments
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.default_with_comments.instruction.current, None);
    }

    #[test]
    fn current_instruction_does_not_change_after_comments() {
        // Arrange
        let mut ctx = setup();

        //Apply
        ctx.default_with_comments
            .advance()
            .expect("Error while calling advance");
        ctx.default_with_comments
            .advance()
            .expect("Error while calling advance");
        ctx.default_with_comments
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert_eq!(
            ctx.default_with_comments.instruction.current,
            Some("@A".to_string())
        );
    }

    #[test]
    fn current_instruction_gets_first_instruction() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.default.advance().expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.default.instruction.current, Some("@A".to_string()));
    }

    #[test]
    fn current_instruction_gets_second_instruction() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.default.advance().expect("Error while calling advance");
        ctx.default.advance().expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.default.instruction.current, Some("@D".to_string()));
    }

    #[test]
    fn instruction_type_before_advance_returns_none() {
        // Arrange
        let ctx = setup();

        // Assert
        assert!(ctx.default.instruction_type().is_none());
    }

    #[test]
    fn instruction_type_returns_a_instruction_for_first_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert!(matches!(
            ctx.a_inst_parser.instruction_type(),
            Some(InstructionType::AInstruction)
        ));
    }

    #[test]
    fn instruction_type_returns_a_instruction_for_second_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert!(matches!(
            ctx.a_inst_parser.instruction_type(),
            Some(InstructionType::AInstruction)
        ));
    }

    #[test]
    fn instruction_type_returns_a_instruction_for_third_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert!(matches!(
            ctx.a_inst_parser.instruction_type(),
            Some(InstructionType::AInstruction)
        ));
    }

    #[test]
    fn instruction_type_returns_c_instruction_for_first_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.c_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert!(matches!(
            ctx.c_inst_parser.instruction_type(),
            Some(InstructionType::CInstruction)
        ));
    }

    #[test]
    fn instruction_type_returns_c_instruction_for_second_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.c_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.c_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert!(matches!(
            ctx.c_inst_parser.instruction_type(),
            Some(InstructionType::CInstruction)
        ));
    }

    #[test]
    fn instruction_type_returns_c_instruction_for_third_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.c_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.c_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.c_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert!(matches!(
            ctx.c_inst_parser.instruction_type(),
            Some(InstructionType::CInstruction)
        ));
    }

    #[test]
    fn instruction_type_returns_l_instruction_for_first_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert!(matches!(
            ctx.l_inst_parser.instruction_type(),
            Some(InstructionType::LInstruction)
        ));
    }

    #[test]
    fn instruction_type_returns_l_instruction_for_second_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert!(matches!(
            ctx.l_inst_parser.instruction_type(),
            Some(InstructionType::LInstruction)
        ));
    }

    #[test]
    fn instruction_type_returns_l_instruction_for_third_call() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");
        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        println!(
            "Expected LInstruction got : {:?}",
            ctx.l_inst_parser.instruction_type()
        );
        assert!(matches!(
            ctx.l_inst_parser.instruction_type(),
            Some(InstructionType::LInstruction)
        ));
    }

    #[test]
    fn symbol_before_advance_returns_none() {
        // Arrange
        let ctx = setup();

        // Assert
        assert!(ctx.a_inst_parser.symbol().is_none());
    }

    #[test]
    fn symbol_retruns_a_instruction_for_one_letter_symbol() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.a_inst_parser.symbol(), Some("D"))
    }

    #[test]
    fn symbol_retruns_a_instruction_for_multiple_digit_symbol() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.a_inst_parser.symbol(), Some("123"));
    }

    #[test]
    fn symbol_a_instruction_for_variable_name() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.a_inst_parser.symbol(), Some("abc"));
    }

    #[test]
    fn symbol_retruns_a_instruction_for_one_letter_variable_name() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        ctx.a_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.a_inst_parser.symbol(), Some("y"))
    }

    #[test]
    fn symbol_retruns_l_instruction_for_first_symbol() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.l_inst_parser.symbol(), Some("123"));
    }

    #[test]
    fn symbol_retruns_l_instruction_for_second_symbol() {
        // Arrange
        let mut ctx = setup();

        // Apply
        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");

        ctx.l_inst_parser
            .advance()
            .expect("Error while calling advance");

        // Assert
        assert_eq!(ctx.l_inst_parser.symbol(), Some("LOOP"));
    }
}
