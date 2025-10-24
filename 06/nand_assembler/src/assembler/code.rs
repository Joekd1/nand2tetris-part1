use std::collections::HashMap;

pub struct Code {
    pub dest_map: HashMap<String, String>,
    pub comp_map: HashMap<String, String>,
    pub jump_map: HashMap<String, String>,
}

impl Code {
    pub fn new() -> Self {
        // Destination
        let dest_map = HashMap::from([
            (String::from("null"), String::from("000")),
            (String::from("M"), String::from("001")),
            (String::from("D"), String::from("010")),
            (String::from("DM"), String::from("011")),
            (String::from("A"), String::from("100")),
            (String::from("AM"), String::from("101")),
            (String::from("AD"), String::from("110")),
            (String::from("ADM"), String::from("111")),
        ]);

        // Computation
        let comp_map = HashMap::from([
            (String::from("0"), String::from("0101010")),
            (String::from("1"), String::from("0111111")),
            (String::from("-1"), String::from("0111010")),
            (String::from("D"), String::from("0001100")),
            (String::from("A"), String::from("0110000")),
            (String::from("!D"), String::from("0001101")),
            (String::from("!A"), String::from("0110001")),
            (String::from("-D"), String::from("0001111")),
            (String::from("-A"), String::from("0110011")),
            (String::from("D+1"), String::from("0011111")),
            (String::from("A+1"), String::from("0110111")),
            (String::from("D-1"), String::from("0001110")),
            (String::from("A-1"), String::from("0110010")),
            (String::from("D+A"), String::from("0000010")),
            (String::from("D-A"), String::from("0010011")),
            (String::from("A-D"), String::from("0000111")),
            (String::from("D&A"), String::from("0000000")),
            (String::from("D|A"), String::from("0010101")),
            (String::from("M"), String::from("1110000")),
            (String::from("!M"), String::from("1110001")),
            (String::from("-M"), String::from("1110011")),
            (String::from("M+1"), String::from("1110111")),
            (String::from("M-1"), String::from("1110010")),
            (String::from("D+M"), String::from("1000010")),
            (String::from("D-M"), String::from("1010011")),
            (String::from("M-D"), String::from("1000111")),
            (String::from("D&M"), String::from("1000000")),
            (String::from("D|M"), String::from("1010101")),
        ]);

        // Jump
        let jump_map = HashMap::from([
            (String::from("null"), String::from("000")),
            (String::from("JGT"), String::from("001")),
            (String::from("JEQ"), String::from("010")),
            (String::from("JGE"), String::from("011")),
            (String::from("JLT"), String::from("100")),
            (String::from("JNE"), String::from("101")),
            (String::from("JLE"), String::from("110")),
            (String::from("JMP"), String::from("111")),
        ]);

        Self {
            dest_map,
            comp_map,
            jump_map,
        }
    }

    fn canonicalize_dest(dest: &str) -> String {
        // "null" is already in its canonical form
        if dest == "null" {
            return String::from("null");
        }

        let mut chars: Vec<char> = dest.chars().collect();
        chars.sort_unstable();
        chars.into_iter().collect()
    }

    pub fn dest(&self, dest: &str) -> Option<&str> {
        let canonical_dest = Code::canonicalize_dest(dest);
        self.dest_map.get(&canonical_dest).map(|s| s.as_str())
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dest_for_single_letter_destination() {
        let code = Code::new();
        assert_eq!(code.dest("M"), Some("001"));
        assert_eq!(code.dest("D"), Some("010"));
        assert_eq!(code.dest("A"), Some("100"));
    }

    #[test]
    fn dest_for_canonical_two_letters_destination() {
        let code = Code::new();
        assert_eq!(code.dest("DM"), Some("011"));
        assert_eq!(code.dest("AM"), Some("101"));
        assert_eq!(code.dest("AD"), Some("110"));
    }

    #[test]
    fn dest_handles_two_letter_permutations() {
        let code = Code::new();
        assert_eq!(code.dest("MD"), Some("011"));
        assert_eq!(code.dest("MA"), Some("101"));
        assert_eq!(code.dest("DA"), Some("110"));
    }

    #[test]
    fn dest_for_canonical_three_letter_destination() {
        let code = Code::new();
        assert_eq!(code.dest("AMD"), Some("111"));
    }

    #[test]
    fn dest_handles_three_letter_permutations() {
        let code = Code::new();
        assert_eq!(code.dest("ADM"), Some("111"));
        assert_eq!(code.dest("DAM"), Some("111"));
        assert_eq!(code.dest("DMA"), Some("111"));
        assert_eq!(code.dest("MAD"), Some("111"));
        assert_eq!(code.dest("MDA"), Some("111"));
    }
    #[test]
    fn dest_for_null_destination() {
        let code = Code::new();
        assert_eq!(code.dest("null"), Some("000"));
    }

    #[test]
    fn dest_for_invalid_input_returns_none() {
        let code = Code::new();
        assert_eq!(code.dest("Z"), None);
        assert_eq!(code.dest("AZD"), None);
        assert_eq!(code.dest("a"), None);
        assert_eq!(code.dest(""), None);
    }
}
