#[allow(non_snake_case)]

use std::process;
use std::vec;
use crate::um::UniversalMachine;
use crate::disassembler::{get, Field, RA, RB, RC, RL, VL};
use std::io::{self, stdin};
use std::num::Wrapping;

/// if r[c] != 0, then r[A] := r[B]
pub fn cmov(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    if UM.r[C as usize] != 0 {
        UM.r[A as usize] = UM.r[B as usize];
    }
}

/// r[a] := m[r[B]][r[C]]
pub fn seg_load(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    let rb_val = UM.r[B as usize] as usize;
    let rc_val = UM.r[C as usize] as usize;
    UM.r[A as usize] = UM.segments[rb_val][rc_val];
}

/// m[r[A]][r[B]] := r[C]
pub fn seg_store(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    let ra_val = UM.r[A as usize] as usize;
    let rb_val = UM.r[B as usize] as usize;
    let rc_val = UM.r[C as usize];
    UM.segments[ra_val][rb_val] = rc_val;
}

/// r[A] := (r[B] + r[C]) mod 2^32
pub fn add(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    // UM.r[A as usize] = UM.r[B as usize] + UM.r[C as usize];
    UM.r[A as usize] = Wrapping(UM.r[B as usize] + UM.r[C as usize]).0; // What's happening here? Good Question.
}

/// r[A] := (r[B] * r[C]) mod 2^32
pub fn mul(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    // UM.r[A as usize] = UM.r[B as usize] * UM.r[C as usize];
    UM.r[A as usize] = Wrapping(UM.r[B as usize] * UM.r[C as usize]).0; // What's happening here? Good Question.

}

/// r[A] := (r[B] / r[C]) mod 2^32
pub fn div(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    // UM.r[A as usize] = UM.r[B as usize] / UM.r[C as usize]; 
    UM.r[A as usize] = Wrapping(UM.r[B as usize] / UM.r[C as usize]).0; // What's happening here? Good Question.
}

/// r[A] := ~(r[B] ^ r[C]) mod 2^32
pub fn nand(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    // UM.r[A as usize] = !(UM.r[B as usize] ^ UM.r[C as usize]);
    UM.r[A as usize] = Wrapping(!(UM.r[B as usize] ^ UM.r[C as usize])).0;
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
        let unmapped_seg_index = *(UM.free_segs.get(0).unwrap());
        UM.segments[unmapped_seg_index as usize] = new_segment;
        UM.r[B as usize] = unmapped_seg_index;
    }
    else {
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
    println!("{}", UM.r[C as usize]);
}

// The UM waits for input on the I/O device. When
// input arrives, $r[c] is loaded with the input,
// which must be a value from 0 to 255. If the end
// of input has been signaled, then $r[C] is loaded
// with a full 32-bit word in which every bit is 1.
pub fn input(UM: &mut UniversalMachine, C: u32) {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).expect("Invalid input!");

    // Input must be value from 0 to 255
    let input_val: u32 = input_string.parse().expect("Invalid input!");
    if input_val <= 255 {
        UM.r[C as usize] = input_val;
    }
}

// Segment $m[$r[B]] is duplicated, and the
// duplicate replaces $m[0], which is abandoned.
// The program counter is set to point to
// $m[0][$r[C]]. If $r[B]=0, the load program
// operation should be extremely quick, as this is
// effectively a jump.
pub fn load_program(UM: &mut UniversalMachine, B: u32, C: u32) {
    UM.segments[0] = UM.segments[B as usize].clone();
    UM.program_counter = UM.segments[0][UM.r[C as usize] as usize]
}

/// Store value found in 25 least signficant bits into r[X],
/// where X is 3 bits wide, LSB 26 (which is 3 bits less signficant than opcode field)
pub fn load_value(UM: &mut UniversalMachine, C: u32) {
    
}