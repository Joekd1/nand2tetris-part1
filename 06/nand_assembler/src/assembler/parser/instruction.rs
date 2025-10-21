pub struct Instruction {
    pub current: Option<String>,
}

impl Instruction {
    pub fn new() -> Self {
        Self { current: None }
    }
}
