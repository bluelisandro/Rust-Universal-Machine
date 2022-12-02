use crate::um::UniversalMachine;

// if r[c] != 0, then r[A] := r[B]
pub fn mov(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    if UM.r[C as usize] != 0 {
        UM.r[A as usize] = UM.r[B as usize];
    }
}

// r[a] := m[r[B]][r[C]]
pub fn load(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {
    UM.r[A as usize] = UM.segments[UM.r[B as usize] as usize][UM.r[B as usize] as usize];
}

// m[r[A]][r[B]] := r[C]
pub fn store(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {}

// r[A] := (r[B] + r[C]) mod 2^32
pub fn add(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {}

// r[A] := (r[B] + r[C]) mod 2^32
pub fn mul(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {}

pub fn div(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {}

pub fn nand(UM: &mut UniversalMachine, A: u32, B: u32, C: u32) {}

pub fn halt() {}

// A new segment is created with a number of words
// equal to the value in $r[C]. Each word in the
// new segment is initialized to zero. A bit pattern
// that is not all zeroes and does not identify any
// currently mapped segment is placed in $r[B]
pub fn map(UM: &mut UniversalMachine, B: u32, C: u32) {

}

// The new segment is mapped as $m[$r[B]].
// The segment $m[$r[C]] is unmapped.
// Future Map Segment instructions may reuse the
// identifier $r[C].
pub fn unmap(UM: &mut UniversalMachine, C: u32) {}
