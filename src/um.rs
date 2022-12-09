pub struct UniversalMachine {
    // Registers vector, where each index represents a register
    pub r: Vec<u32>,

    // Segments vector, where each vector represents an i'th segment
    // Each i'th segment is a vector containing u32 words
    // Inside each i'th segment, the n'th offset can be accessed within that segment's vector at the n'th index
    pub segments: Vec<Vec<u32>>,
    pub free_segs: Vec<u32>,

    // Program counter
    pub program_counter: u32,
}

impl UniversalMachine {
    pub fn new() -> Self {
        Self {
            r: vec![0, 0, 0, 0, 0, 0, 0, 0],
            segments: vec![vec![0; 10]],
            free_segs: Vec::new(),
            program_counter: 0,
        }
    }
}
