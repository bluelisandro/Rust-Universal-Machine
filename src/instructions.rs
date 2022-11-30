// if r[c] != 0, then r[A] := r[B]
pub fn conditional_move(A: u32, B: u32, C: u32) {
    
}

// r[a] := m[r[B]][r[C]]
pub fn segmented_load() {
    
}

// m[r[A]][r[B]] := r[C]
pub fn segmented_store() {
    
}

// r[A] := (r[B] + r[C]) mod 2^32
pub fn addition() {
    
}

// r[A] := (r[B] + r[C]) mod 2^32
pub fn multiplication() {
    
}

pub fn division() {
    
}

pub fn bitwise_nand() {
    
}

pub fn halt() {
    
}

// A new segment is created with a number of words
// equal to the value in $r[C]. Each word in the
// new segment is initialized to zero. A bit pattern
// that is not all zeroes and does not identify any
// currently mapped segment is placed in $r[B]
pub fn map_segment() {
    
}

// The new segment is mapped as $m[$r[B]].
// The segment $m[$r[C]] is unmapped.
// Future Map Segment instructions may reuse the
// identifier $r[C].
pub fn unmap_segment() {
    
}