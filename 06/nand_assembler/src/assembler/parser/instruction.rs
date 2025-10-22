use regex::Regex;

#[derive(Debug)]
pub enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

pub struct InstructionRegex {
    pub reg_a: Regex,
    pub reg_c: Regex,
    pub reg_l: Regex,
}

pub struct Instruction {
    pub current: Option<String>,
    pub regex: InstructionRegex,
}

impl InstructionRegex {
    pub fn new() -> Self {
        let reg_a = Regex::new(r"^@(([0-9]+)|([a-zA-Z:_.$][a-zA-Z0-9:_.$]*))$").unwrap();
        let reg_c = Regex::new(
            r"^((?<dest>[AMD]+)=)?(?<comp>[-!]?[AMD01]+([-+|&][AMD1]+)?)(;(?<jmp>J[A-Z]{2,3}))?$",
        )
        .unwrap();

        let reg_l = Regex::new(r"^\((([0-9]+)|([a-zA-Z:_.$][a-zA-Z0-9:_.$]*))\)$").unwrap();

        Self {
            reg_a,
            reg_c,
            reg_l,
        }
    }
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            regex: InstructionRegex::new(),
            current: None,
        }
    }
}
