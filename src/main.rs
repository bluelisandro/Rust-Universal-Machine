#![allow(non_snake_case)]
use std::env;
use rum::disassembler;
use rum::rumload;
use rum::um::UniversalMachine;

fn main() {
    let input = env::args().nth(1);
    let mut UM = UniversalMachine::new();
    UM.segments[0] = rumload::load(input.as_deref());
    disassembler::launch(&mut UM);
}
