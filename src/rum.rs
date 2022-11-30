pub struct UniversalMachine {
    // Registers vector, where each index represents a register
    pub registers: Vec<u32>,

    // Segments vector, where each vector represents an i'th segment
    // Each i'th segment is a vector containing u32 words
    // Inside each i'th segment, the n'th offset can be accessed within that segment's vector at the n'th index
    pub segments: Vec<Vec<u32>>,
}

impl UniversalMachine {
    pub fn new() -> Self {
        Self {
            registers: Vec::with_capacity(8),
            segments: vec![Vec::new()]
        }
    }
}
