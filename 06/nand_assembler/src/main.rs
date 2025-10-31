use assembler::Assembler;
mod assembler;

use std::env;
use std::process;

fn main() {
    let mut assembler = Assembler::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = assembler.run() {
        eprintln!("{err}");
    }
}
