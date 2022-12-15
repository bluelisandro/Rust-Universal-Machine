type Umi = u32;
pub struct Field {
    pub width: u32,
    pub lsb: u32,
}

pub static RA: Field = Field { width: 3, lsb: 6 }; // A value
pub static RB: Field = Field { width: 3, lsb: 3 }; // B value
pub static RC: Field = Field { width: 3, lsb: 0 }; // C value
pub static RL: Field = Field { width: 3, lsb: 25 }; // Load program location
pub static VL: Field = Field { width: 25, lsb: 0 }; // Value
pub static OP: Field = Field { width: 4, lsb: 28 }; // Opcode

#[derive(Debug, PartialEq, Clone, Copy)]
enum Opcode {
    CMov,         // 0
    Load,         // 1
    Store,        // 2
    Add,          // 3
    Mul,          // 4
    Div,          // 5
    Nand,         // 6
    Halt,         // 7
    MapSegment,   // 8
    UnmapSegment, // 9
    Output,       // 10
    Input,        // 11
    LoadProgram,  // 12
    LoadValue,    // 13
}

fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

pub fn get(field: &Field, instruction: &Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

pub fn launch(UM: &mut UniversalMachine) {
    loop {
        disassemble(UM);
    }
}

use crate::um::UniversalMachine;
pub fn disassemble(UM: &mut UniversalMachine) {
    use crate::instructions;

    // let instruction = &UM.segments[0][UM.program_counter];
    let instruction = UM.segments.get(0).unwrap().get(UM.program_counter).unwrap();

    // Gets actual integer values for A, B, C from instruction u32 word
    let A_val = get(&RA, instruction);
    let B_val = get(&RB, instruction);
    let C_val = get(&RC, instruction);

    // Increment program counter
    UM.program_counter += 1;

    match get(&OP, instruction) {
        o if o == Opcode::CMov as u32 => {
            instructions::cmov(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Load as u32 => {
            instructions::seg_load(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Store as u32 => {
            instructions::seg_store(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Add as u32 => {
            instructions::add(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Mul as u32 => {
            instructions::mul(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Div as u32 => {
            instructions::div(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Nand as u32 => {
            instructions::nand(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Halt as u32 => {
            instructions::halt();
        }

        o if o == Opcode::MapSegment as u32 => {
            instructions::map_seg(UM, B_val, C_val);
        }

        o if o == Opcode::UnmapSegment as u32 => {
            instructions::unmap_seg(UM, C_val);
        }

        o if o == Opcode::Output as u32 => {
            instructions::output(UM, C_val);
        }

        o if o == Opcode::Input as u32 => {
            instructions::input(UM, C_val);
        }

        o if o == Opcode::LoadProgram as u32 => {
            UM.program_counter -= 1;
            instructions::load_program(UM, B_val, C_val);
        }

        o if o == Opcode::LoadValue as u32 => {
            instructions::load_value(UM, *instruction);
        }

        _ => ()
    }
}
