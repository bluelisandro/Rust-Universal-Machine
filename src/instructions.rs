use crate::rum::UniversalMachine;

// if r[c] != 0, then r[A] := r[B]
pub fn conditional_move(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    if UM.registers[0] != 0 {
        UM.registers[A as usize] = UM.registers[B as usize];
    }
}

// r[a] := m[r[B]][r[C]]
pub fn segmented_load(A: u32, B: u32, C: u32) {}

// m[r[A]][r[B]] := r[C]
pub fn segmented_store(A: u32, B: u32, C: u32) {}

// r[A] := (r[B] + r[C]) mod 2^32
pub fn addition(A: u32, B: u32, C: u32) {}

// r[A] := (r[B] + r[C]) mod 2^32
pub fn multiplication(A: u32, B: u32, C: u32) {}

pub fn division(A: u32, B: u32, C: u32) {}

pub fn bitwise_nand(A: u32, B: u32, C: u32) {}

pub fn halt() {}

// A new segment is created with a number of words
// equal to the value in $r[C]. Each word in the
// new segment is initialized to zero. A bit pattern
// that is not all zeroes and does not identify any
// currently mapped segment is placed in $r[B]
pub fn map_segment(B: u32, C: u32) {

}

// The new segment is mapped as $m[$r[B]].
// The segment $m[$r[C]] is unmapped.
// Future Map Segment instructions may reuse the
// identifier $r[C].
pub fn unmap_segment(C: u32) {}
