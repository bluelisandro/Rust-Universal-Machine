fn conditional_move() {
    
}

fn segmented_load() {
    
}

fn segmented_store() {
    
}

fn addition() {
    
}

fn multiplication() {
    
}

fn division() {
    
}

fn bitwise_nand() {
    
}

fn halt() {
    
}

// A new segment is created with a number of words
// equal to the value in $r[C]. Each word in the
// new segment is initialized to zero. A bit pattern
// that is not all zeroes and does not identify any
// currently mapped segment is placed in $r[B]
fn map_segment() {
    
}

// The new segment is mapped as $m[$r[B]].
// The segment $m[$r[C]] is unmapped.
// Future Map Segment instructions may reuse the
// identifier $r[C].
fn unmap_segment() {
    
}