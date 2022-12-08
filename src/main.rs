#![allow(non_snake_case)]
use std::env;
use Rust_Universal_Machine::disassembler;
use Rust_Universal_Machine::rumload;
use Rust_Universal_Machine::um::UniversalMachine;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    let mut cpu0 = UniversalMachine::new();

    for instruction in instructions {
        println!("{}", disassembler::disassemble(&mut cpu0, instruction));
    }

    // println!("{} instructions", instructions.len());

    // for instruction in instructions {
    //     println!("{}", disassembler::disassemble(instruction));
    // }
}
