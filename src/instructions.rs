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

// A new segment is created with a number of words
// equal to the value in $r[C]. Each word in the
// new segment is initialized to zero. A bit pattern
// that is not all zeroes and does not identify any
// currently mapped segment is placed in $r[B]
pub fn map(UM: &mut UniversalMachine, B: u32, C: u32) {
    // Push a new vector with rc_val zeroes to segments
    let rc_val = get(&RC, UM.r[C as usize]);

    // Check if we already have any empty segments
    


    // UM.segments.push(vec![0; rc_val as usize]);
}

// The new segment is mapped as $m[$r[B]].
// The segment $m[$r[C]] is unmapped.
// Future Map Segment instructions may reuse the
// identifier $r[C].
pub fn unmap(UM: &mut UniversalMachine, C: u32) {
    
}
