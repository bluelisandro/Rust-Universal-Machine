type Umi = u32;
pub struct Field {
    width: u32,
    lsb: u32,
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

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

// inc program counter before instruction is executed !!!

use crate::um::UniversalMachine;
pub fn disassemble(UM: &mut UniversalMachine, instruction: Umi) -> String {
    use crate::instructions;
 
    // Gets actual integer values for A, B, C from instruction u32 word
    let A_val = get(&RA, instruction);
    let B_val = get(&RB, instruction);
    let C_val = get(&RC, instruction);

    match get(&OP, instruction) {
        o if o == Opcode::CMov as u32 => {
            instructions::mov(UM, A_val, B_val, C_val);
            format!(
                "if (r{} != 0) r{} := r{};",
                get(&RC, instruction),
                get(&RA, instruction),
                get(&RB, instruction)
            )
        }

        o if o == Opcode::Load as u32 => {
            instructions::load(UM, A_val, B_val, C_val);
            format!("Load r{};", get(&RL, instruction),)
        }

        o if o == Opcode::Store as u32 => {
            instructions::store(UM, A_val, B_val, C_val);
            format!("Store r{};", get(&VL, instruction),)
        }

        o if o == Opcode::Add as u32 => {
            instructions::add(UM, A_val, B_val, C_val);
            format!(
                "r{} := r{} + r{};",
                get(&RA, instruction),
                get(&RB, instruction),
                get(&RC, instruction)
            )
        }

        o if o == Opcode::Mul as u32 => {
            instructions::mul(UM, A_val, B_val, C_val);
            format!(
                "r{} := r{} * r{};",
                get(&RA, instruction),
                get(&RB, instruction),
                get(&RC, instruction)
            )
        }

        o if o == Opcode::Div as u32 => {
            instructions::div(UM, A_val, B_val, C_val);
            format!(
                "r{} := r{} / r{};",
                get(&RA, instruction),
                get(&RB, instruction),
                get(&RC, instruction)
            )
        }

        o if o == Opcode::Nand as u32 => {
            instructions::nand(UM, A_val, B_val, C_val);
            format!(
                "r{} := ~(r{} ^ r{});",
                get(&RA, instruction),
                get(&RB, instruction),
                get(&RC, instruction)
            )
        }

        o if o == Opcode::Halt as u32 => {
            instructions::halt();
            format!("HALT;")
        }

        o if o == Opcode::MapSegment as u32 => {
            instructions::map(UM, B_val, C_val);
            format!("MAP SEG r{};", get(&RC, instruction),)
        }

        o if o == Opcode::UnmapSegment as u32 => {
            instructions::unmap(UM, C_val);
            format!("UNMAP r{};", get(&RC, instruction),)
        }

        o if o == Opcode::Output as u32 => {
            format!("OUTPUT: r{};", get(&RC, instruction),)
        }

        o if o == Opcode::Input as u32 => {
            format!("INPUT: r{};", get(&RC, instruction),)
        }

        o if o == Opcode::LoadProgram as u32 => {
            format!("LOAD PROGRAM r{};", get(&RB, instruction))
        }

        o if o == Opcode::LoadValue as u32 => {
            format!("LOAD VALUE r{};", get(&VL, instruction),)
        }

        _ => format!("ERROR: INVALID OPCODE!"),
    }
}
