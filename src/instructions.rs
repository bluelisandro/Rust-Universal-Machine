#[allow(non_snake_case)]

use std::process;
use std::vec;
use crate::um::UniversalMachine;
use crate::disassembler::{get, Field, RA, RB, RC, RL, VL};

// if r[c] != 0, then r[A] := r[B]
pub fn mov(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    if UM.r[C as usize] != 0 {
        UM.r[A as usize] = UM.r[B as usize];
    }
}

// r[a] := m[r[B]][r[C]]
pub fn load(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    let rb_val = UM.r[B as usize] as usize;
    let rc_val = UM.r[C as usize] as usize;
    UM.r[A as usize] = UM.segments[rb_val][rc_val];
}

// m[r[A]][r[B]] := r[C]
pub fn store(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    let ra_val = UM.r[A as usize] as usize;
    let rb_val = UM.r[B as usize] as usize;
    let rc_val = UM.r[C as usize];
    UM.segments[ra_val][rb_val] = rc_val;
}

// r[A] := (r[B] + r[C]) mod 2^32
pub fn add(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    UM.r[A as usize] = UM.r[B as usize] + UM.r[C as usize];
}

// r[A] := (r[B] * r[C]) mod 2^32
pub fn mul(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    UM.r[A as usize] = UM.r[B as usize] * UM.r[C as usize];
}

// r[A] := (r[B] / r[C]) mod 2^32
pub fn div(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    UM.r[A as usize] = UM.r[B as usize] / UM.r[C as usize]; 
}

// r[A] := ~(r[B] ^ r[C]) mod 2^32
pub fn nand(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    UM.r[A as usize] = !(UM.r[B as usize] ^ UM.r[C as usize]);
}

// Stop UM
pub fn halt() {
    process::exit(0);
}

pub fn map(UM: &mut UniversalMachine, B: u32, C: u32) {
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

pub fn unmap(UM: &mut UniversalMachine, C: u32) {
    // To unmap a segment, simply add its index to the free_segs vector
    UM.free_segs.push(UM.r[C as usize]);
}
