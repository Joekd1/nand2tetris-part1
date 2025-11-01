use code::Code;
use parser::Parser;
use parser::instruction::InstructionType;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use symbol_table::SymbolTable;

pub struct Assembler {
    file_path: String,
    symbol_table: SymbolTable,
    parser: Parser,
    code: Code,
    next_address: i32,
    binary: Vec<String>,
}

mod code;
mod parser;
mod symbol_table;

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

        let symbol_table = SymbolTable::new();
        let parser = Parser::build(&file_path)?;
        let code = Code::new();
        let binary: Vec<String> = Vec::new();
        let next_address = 16;

        Ok(Assembler {
            file_path,
            symbol_table,
            parser,
            code,
            next_address,
            binary,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.build_symbol_table()?;

	self.parser.file_index = -1;
	self.parser.current_line = -1;

        while let Ok(()) = self.parser.advance() {
            if let Some(inst_type) = self.parser.instruction_type() {
                match inst_type {
                    InstructionType::AInstruction => self.assemble_a_instruction(),
                    InstructionType::CInstruction => self.assemble_c_instruction()?,
                    InstructionType::LInstruction => continue,
                }
            }
        }

        self.write_binary_to_file()?;
        Ok(())
    }

    fn build_symbol_table(&mut self) -> Result<(), Box<dyn Error>> {
        while let Ok(()) = self.parser.advance() {
            self.parser
                .instruction_type()
                .filter(|t| matches!(t, InstructionType::LInstruction))
                .and_then(|_| self.parser.symbol())
                .map(|symbol| {
                    self.symbol_table
                        .add_entry(symbol, self.parser.current_line + 1)
                });
        }
        Ok(())
    }

    fn assemble_a_instruction(&mut self) {
        if let Some(binary_part) = self.parser.symbol()
	    .map(|s| s.to_string())
            .and_then(|s| self.process_symbol(&s))
            .map(|n| Assembler::convert_number_to_binary(&n))
        {
            self.binary.push(format!("0{}", binary_part));
        }
    }

    fn process_symbol(&mut self, s: &str) -> Option<i32> {
        if let Ok(n) = s.parse::<i32>() {
            return Some(n);
        }
        if self.symbol_table.contains(s) {
            self.symbol_table.get_address(s)
        } else {
            let address = self.next_address;
            self.symbol_table.add_entry(s, address);
            self.next_address += 1;
            Some(address)
        }
    }

    fn convert_number_to_binary(number: &i32) -> String {
        format!("{:015b}", number) // 15-bit binary padding
    }

    fn assemble_c_instruction(&mut self) -> Result<(), Box<dyn Error>> {
        let comp_string = self
            .parser
            .comp()
            .and_then(|c| self.code.comp(c))
            .ok_or("Error assembling the comp part")?;

        let dest_string = self
            .parser
            .dest()
            .and_then(|d| self.code.dest(d))
            .unwrap_or("000"); // Default to "000" if no dest

        let jump_string = self
            .parser
            .jump()
            .and_then(|j| self.code.jump(j))
            .unwrap_or("000"); // Default to "000" if no jump

        self.binary
            .push(format!("111{}{}{}", comp_string, dest_string, jump_string));

        Ok(())
    }

    fn write_binary_to_file(&self) -> Result<(), std::io::Error> {
        let filename = self.get_output_path();
        let file = File::create(&filename)?;
        let mut writer = BufWriter::new(file);

        let content = self.binary.join("\n");

        writer.write_all(content.as_bytes())?;

        Ok(())
    }

    fn get_output_path(&self) -> PathBuf {
        PathBuf::from(&self.file_path).with_extension("hack")
    }
}
