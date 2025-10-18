use parser::Parser;
use std::error::Error;
use std::path::Path;

pub struct Assembler {
    parser: Parser,
}

mod parser;

impl Assembler {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        // The first value is the name of the program
        args.next();

        let file_path = args.next().ok_or("Didn't get file path").and_then(|path| {
            Path::new(&path)
                .is_file()
                .then_some(Path::new(&path))
                .ok_or("File doesn't exist")?
                .extension()
                .filter(|&ext| ext == "asm")
                .map(|_| path.clone())
                .ok_or("Wrong file extension")
        })?;

        let parser = Parser::build(&file_path)?;

        Ok(Assembler { parser })
    }
}
