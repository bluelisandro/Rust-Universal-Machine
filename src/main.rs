#![allow(non_snake_case)]
use rum::rumload;
use std::env;

pub struct UniversalMachine {
    pub r: Vec<u32>,
    pub segments: Vec<Vec<u32>>,
    pub free_segs: Vec<u32>,

    // Program counter
    pub program_counter: usize,
}

impl UniversalMachine {
    pub fn new() -> Self {
        Self {
            r: vec![0; 8],
            segments: vec![vec![]],
            free_segs: vec![],
            program_counter: 0,
        }
    }
}

fn main() {
    let input = env::args().nth(1);
    let mut UM = UniversalMachine::new();
    UM.segments[0] = rumload::load(input.as_deref());
    loop {
        disassemble(&mut UM);
    }
}

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
    (*instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

// use crate::um::UniversalMachine;
pub fn disassemble(UM: &mut UniversalMachine) {
    // use crate::instructions;

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
            cmov(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Load as u32 => {
            seg_load(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Store as u32 => {
            seg_store(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Add as u32 => {
            add(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Mul as u32 => {
            mul(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Div as u32 => {
            div(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Nand as u32 => {
            nand(UM, A_val, B_val, C_val);
        }

        o if o == Opcode::Halt as u32 => {
            halt();
        }

        o if o == Opcode::MapSegment as u32 => {
            map_seg(UM, B_val, C_val);
        }

        o if o == Opcode::UnmapSegment as u32 => {
            unmap_seg(UM, C_val);
        }

        o if o == Opcode::Output as u32 => {
            output(UM, C_val);
        }

        o if o == Opcode::Input as u32 => {
            input(UM, C_val);
        }

        o if o == Opcode::LoadProgram as u32 => {
            UM.program_counter -= 1;
            load_program(UM, B_val, C_val);
        }

        o if o == Opcode::LoadValue as u32 => {
            load_value(UM, *instruction);
        }

        _ => (),
    }
}

use std::io::*;
use std::process;
use std::vec;

/// if $r[C]= 0 then $r[A] := $r[B]
/// Set value at register A equal to the value at register B if and only if the value at
/// register C does not equal 0.
pub fn cmov(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    if UM.r[C as usize] != 0 {
        UM.r[A as usize] = UM.r[B as usize];
    }
}

/// r[A] := m[r[B]][r[C]]
/// Set the value at register A equal to the value at segment [r[B]][r[C]]
pub fn seg_load(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    let rb_val = UM.r[B as usize] as usize;
    let rc_val = UM.r[C as usize] as usize;
    UM.r[A as usize] = UM.segments[rb_val][rc_val];
}

/// m[r[A]][r[B]] := r[C]
/// Set the value of the segment at index r
pub fn seg_store(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    let ra_val = UM.r[A as usize] as usize;
    let rb_val = UM.r[B as usize] as usize;
    let rc_val = UM.r[C as usize];
    UM.segments[ra_val][rb_val] = rc_val;
}

/// r[A] := (r[B] + r[C]) mod 2^32
/// Add the values in registers B and C together and stores it into register A
pub fn add(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    UM.r[A as usize] = UM.r[B as usize].wrapping_add(UM.r[C as usize]);
}

/// r[A] := (r[B] * r[C]) mod 2^32
/// Multiples the values in registers B and C and stores it into register A
pub fn mul(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    UM.r[A as usize] = UM.r[B as usize].wrapping_mul(UM.r[C as usize]);
}

/// r[A] := (r[B] / r[C]) mod 2^32
/// Divides the value at register B by the value at register C (must not equal 0) and stores it into register A
pub fn div(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    if UM.r[C as usize] == 0 {
        panic!("Dividing by 0!");
    }

    UM.r[A as usize] = UM.r[B as usize].wrapping_div(UM.r[C as usize]);
}

/// r[A] := ~(r[B] ^ r[C]) mod 2^32
/// Perform bitwise AND operation and negates it. Stores result into register A
pub fn nand(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    UM.r[A as usize] = !(UM.r[B as usize] & UM.r[C as usize]);
}

/// Exit RUM
pub fn halt() {
    process::exit(0);
}

/// Map new segment of size r[C] words, and store newly mapped segment index in r[B]
pub fn map_seg(UM: &mut UniversalMachine, B: u32, C: u32) {
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

/// Unmap segment in r[c].
/// Does not actually modify values, only adds segment index at r[C] to list of free segment indices.
/// Future Map Segment instructions may reuse the identifier r[C].
pub fn unmap_seg(UM: &mut UniversalMachine, C: u32) {
    // To unmap a segment, simply add its index to the free_segs vector
    UM.free_segs.push(UM.r[C as usize]);
}

/// The value in $r[C] is displayed on the I/O device immediately.
/// Only values from 0 to 255 are allowed.
pub fn output(UM: &mut UniversalMachine, C: u32) {
    // print!("{}", char::from_u32(UM.r[C as usize]).unwrap());
    let r = u8::try_from(UM.r[C as usize]).unwrap();
    print!("{}", r as char);
}

/// The UM waits for input on the I/O device. When
/// input arrives, $r[c] is loaded with the input,
/// which must be a value from 0 to 255. If the end
/// of input has been signaled, then $r[C] is loaded
/// with a full 32-bit word in which every bit is 1.
pub fn input(UM: &mut UniversalMachine, C: u32) {
    match stdin().bytes().next() {
        Some(value) => UM.r[C as usize] = value.unwrap() as u32,
        None => UM.r[C as usize] = !0 as u32,
    }
}

// Segment $m[$r[B]] is duplicated, and the
// duplicate replaces $m[0], which is abandoned.
// The program counter is set to point to
// $m[0][$r[C]]. If $r[B]=0, the load program
// operation should be extremely quick, as this is
// effectively a jump.
pub fn load_program(UM: &mut UniversalMachine, B: u32, C: u32) {
    if UM.r[B as usize] == 0 {
        UM.program_counter = UM.r[C as usize] as usize;
    } else {
        *UM.segments.get_mut(0).unwrap() = UM.segments[UM.r[B as usize] as usize].clone();
        UM.program_counter = UM.r[C as usize] as usize;
    }
}

/// Load Y into r[X].
/// Where X is the 3 bits less significant than the opcode field, which represents a register.
/// Where Y is the remaining 25 bits, which represent a value.
pub fn load_value(UM: &mut UniversalMachine, word: u32) {
    let X = get(&RL, &word);
    let Y = get(&VL, &word);

    UM.r[X as usize] = Y;
}
