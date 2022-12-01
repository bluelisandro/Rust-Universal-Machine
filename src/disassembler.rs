type Umi = u32;
pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field { width: 3, lsb: 6 }; // A value
static RB: Field = Field { width: 3, lsb: 3 }; // B value
static RC: Field = Field { width: 3, lsb: 0 }; // C value
static RL: Field = Field { width: 3, lsb: 25 }; // Load program location
static VL: Field = Field { width: 25, lsb: 0 }; // Value
static OP: Field = Field { width: 4, lsb: 28 }; // Opcode

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

use crate::rum::UniversalMachine;
pub fn disassemble(UM: &mut UniversalMachine, inst: Umi) -> String {
    use crate::instructions;

    let A_val = get(&RA, inst);
    let B_val = get(&RB, inst);
    let C_val = get(&RC, inst);

    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            instructions::conditional_move(UM, A_val, B_val, C_val);
            format!(
                "if (r{} != 0) r{} := r{};",
                get(&RC, inst),
                get(&RA, inst),
                get(&RB, inst)
            )
        }

        o if o == Opcode::Load as u32 => {
            instructions::segmented_load(A_val, B_val, C_val);
            format!("Load r{};", get(&RL, inst),)
        }

        o if o == Opcode::Store as u32 => {
            instructions::segmented_store(A_val, B_val, C_val);
            format!("Store r{};", get(&VL, inst),)
        }

        o if o == Opcode::Add as u32 => {
            instructions::addition(A_val, B_val, C_val);
            format!(
                "r{} := r{} + r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        o if o == Opcode::Mul as u32 => {
            instructions::multiplication(A_val, B_val, C_val);
            format!(
                "r{} := r{} * r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        o if o == Opcode::Div as u32 => {
            instructions::division(A_val, B_val, C_val);
            format!(
                "r{} := r{} / r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        o if o == Opcode::Nand as u32 => {
            instructions::bitwise_nand(A_val, B_val, C_val);
            format!(
                "r{} := ~(r{} ^ r{});",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        o if o == Opcode::Halt as u32 => {
            instructions::halt();
            format!("HALT;")
        }

        o if o == Opcode::MapSegment as u32 => {
            instructions::map_segment(B_val, C_val);
            format!("MAP SEG r{};", get(&RC, inst),)
        }

        o if o == Opcode::UnmapSegment as u32 => {
            instructions::unmap_segment(C_val);
            format!("UNMAP r{};", get(&RC, inst),)
        }

        o if o == Opcode::Output as u32 => {
            format!("OUTPUT: r{};", get(&RC, inst),)
        }

        o if o == Opcode::Input as u32 => {
            format!("INPUT: r{};", get(&RC, inst),)
        }

        o if o == Opcode::LoadProgram as u32 => {
            format!("LOAD PROGRAM r{};", get(&RB, inst))
        }

        o if o == Opcode::LoadValue as u32 => {
            format!("LOAD VALUE r{};", get(&VL, inst),)
        }

        _ => format!("ERROR: INVALID OPCODE!"),
    }
}
