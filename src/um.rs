pub struct UniversalMachine {
    /// There are several invariants within our Rust Universal Machine.
    /// The first being the registers as we represent 8 machine registers as a vector. The indices from 0-7 represent each individual register
    /// The second invariant is the segments which represented segmented memory. It is represented as a 2D vector where the first index represents a
    /// memory segment and the second representing an offset at that memory segment. The third invariant is are the Opcode instructions that
    /// the universal machine runs. There are 14 instructions which are Conditional Move, Segmented Load, Segmented Store,
    /// Addition, Multiplication, Division, Bitwise NAND, Halt, Map Segment, Output, Input, Load Program, and Load Value. Our RUM has an individual
    /// function to perform each of these unique operations. A fourth invariant would be our RUM's free_segs vector, where each index in the vector represents
    /// an unmapped memory segments in the machine.
    // Registers vector, where each index represents a register
    pub r: Vec<u32>,

    // Segments vector, where each vector represents an i'th segment
    // Each i'th segment is a vector containing u32 words
    // Inside each i'th segment, the n'th offset can be accessed within that segment's vector at the n'th index
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
