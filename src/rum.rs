type Register = u32;

pub static r0: Register = 0;
pub static r1: Register = 0;
pub static r2: Register = 0;
pub static r3: Register = 0;
pub static r4: Register = 0;
pub static r5: Register = 0;
pub static r6: Register = 0;
pub static r7: Register = 0;

// Segments vector, where each vector represents an i'th segment
    // Each i'th segment is a vector containing u32 words
        // Inside each i'th segment, the n'th offset can be accessed within that segment's vector at the n'th index
type Segment = Vec<u32>;
static segments: Vec<Segment> = Vec::new();

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
    CMov,           // 0
    Load,           // 1
    Store,          // 2
    Add,            // 3
    Mul,            // 4
    Div,            // 5
    Nand,           // 6
    Halt,           // 7
    MapSegment,     // 8
    UnmapSegment,   // 9
    Output,         // 10
    Input,          // 11
    LoadProgram,    // 12
    LoadValue,      // 13
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

pub fn disassemble(inst: Umi) -> String {
    
    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            format!(
                "if (r{} != 0) r{} := r{};",
                get(&RC, inst),
                get(&RA, inst),
                get(&RB, inst)
            )
        }

        o if o == Opcode::Load as u32 => {
            format!(
                "Load r{};",
                get(&RL, inst),
            )
        }

        o if o == Opcode::Store as u32 => {
            format!(
                "Store r{};",
                get(&VL, inst),
            )
        }

        o if o == Opcode::Add as u32 => {
            format!(
                "r{} := r{} + r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        o if o == Opcode::Mul as u32 => {
            format!(
                "r{} := r{} * r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        o if o == Opcode::Div as u32 => {
            format!(
                "r{} := r{} / r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        o if o == Opcode::Nand as u32 => {
            format!(
                "r{} := ~(r{} ^ r{});",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        o if o == Opcode::Halt as u32 => {
            format!(
                "HALT;"
            )
        }

        o if o == Opcode::MapSegment as u32 => {
            format!(
                "MAP SEG r{};",
                get(&RC, inst),
            )
        }

        o if o == Opcode::UnmapSegment as u32 => {
            format!(
                "UNMAP r{};",
                get(&RC, inst),
            )
        }

        o if o == Opcode::Output as u32 => {
            format!(
                "OUTPUT: r{};",
                get(&RC, inst),
            )
        }

        o if o == Opcode::Input as u32 => {
            format!(
                "INPUT: r{};",
                get(&RC, inst),
            )
        }

        o if o == Opcode::LoadProgram as u32 => {
            format!(
                "LOAD PROGRAM r{};",
                get(&RB, inst)
            )
        }

        o if o == Opcode::LoadValue as u32 => {
            format!(
                "LOAD VALUE r{};",
                get(&VL, inst),
            )
        }
        
        _ => format!("ERROR: INVALID OPCODE!")
    }
}