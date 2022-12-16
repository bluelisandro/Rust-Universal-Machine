use crate::um::UniversalMachine;
use std::process;
use std::io::*;

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

pub fn get(field: &Field, instruction: Umi) -> u32 {
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

pub fn disassemble(UM: &mut UniversalMachine) {

    let instruction = UM.segments[0][UM.program_counter];
    // let instruction = *UM.segments.get(0).unwrap().get(UM.program_counter).unwrap();

    // Gets actual integer values for A, B, C from instruction u32 word
    let A = get(&RA, instruction);
    let B = get(&RB, instruction);
    let C = get(&RC, instruction);

    // Increment program counter
    UM.program_counter += 1;

    match get(&OP, instruction) {
        o if o == Opcode::CMov as u32 => {
            if UM.r[C as usize] != 0 {
                UM.r[A as usize] = UM.r[B as usize];
            }
        }

        o if o == Opcode::Load as u32 => {
            let rb_val = UM.r[B as usize] as usize;
            let rc_val = UM.r[C as usize] as usize;
            UM.r[A as usize] = UM.segments[rb_val][rc_val];
        }

        o if o == Opcode::Store as u32 => {
            let ra_val = UM.r[A as usize] as usize;
            let rb_val = UM.r[B as usize] as usize;
            let rc_val = UM.r[C as usize];
            UM.segments[ra_val][rb_val] = rc_val;
        }

        o if o == Opcode::Add as u32 => {
            UM.r[A as usize] = UM.r[B as usize].wrapping_add(UM.r[C as usize]);
        }

        o if o == Opcode::Mul as u32 => {
            UM.r[A as usize] = UM.r[B as usize].wrapping_mul(UM.r[C as usize]);
        }

        o if o == Opcode::Div as u32 => {
            UM.r[A as usize] = UM.r[B as usize].wrapping_div(UM.r[C as usize]);
        }

        o if o == Opcode::Nand as u32 => {
            UM.r[A as usize] = !(UM.r[B as usize] & UM.r[C as usize]);
        }

        o if o == Opcode::Halt as u32 => {
            process::exit(0);
        }

        o if o == Opcode::MapSegment as u32 => {
            // B is where we put the index storing the new segment
            // C is our new segment word length

            // Create a new vector with r[C] words
            let rc_val = UM.r[C as usize];
            let new_segment = vec![0_u32; rc_val as usize];

            // Check if we already have any unmapped segments
            if UM.free_segs.len() > 0 {
                // If we do have an unmapped segment:
                // -Push new segment vector to the unmapped segment index
                // -Store the unmapped segment's index in r[B]
                // let unmapped_seg_index = *(UM.free_segs.get(0).unwrap());
                let unmapped_seg_index = UM.free_segs.pop().unwrap();
                UM.segments[unmapped_seg_index as usize] = new_segment;
                UM.r[B as usize] = unmapped_seg_index;
            } else {
                // If we don't have any empty segments, push a new one to the segments vec
                UM.segments.push(new_segment);

                // The new segment index is the length of the segments vec
                UM.r[B as usize] = (UM.segments.len() - 1) as u32;
            }
        }

        o if o == Opcode::UnmapSegment as u32 => {
            UM.free_segs.push(UM.r[C as usize]);
        }

        o if o == Opcode::Output as u32 => {
            let r = u8::try_from(UM.r[C as usize]).unwrap();
            print!("{}", r as char);
        }

        o if o == Opcode::Input as u32 => match stdin().bytes().next() {
            Some(value) => UM.r[C as usize] = value.unwrap() as u32,
            None => UM.r[C as usize] = !0 as u32,
        },

        o if o == Opcode::LoadProgram as u32 => {
            UM.program_counter -= 1;
            if UM.r[B as usize] == 0 {
                UM.program_counter = UM.r[C as usize] as usize;
            } else {
                *UM.segments.get_mut(0).unwrap() = UM.segments[UM.r[B as usize] as usize].clone();
                UM.program_counter = UM.r[C as usize] as usize;
            }
        }

        o if o == Opcode::LoadValue as u32 => {
            let X = get(&RL, instruction);
            let Y = get(&VL, instruction);

            UM.r[X as usize] = Y;
        }

        _ => (),
    }
}
