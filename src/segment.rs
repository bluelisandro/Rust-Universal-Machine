// Segments vector, where each vector represents an i'th segment
    // Each i'th segment is a vector containing u32 words
        // Inside each i'th segment, the n'th offset can be accessed within that segment's vector at the n'th index
static segments: Vec<Vec<u32>> = Vec::new();

// Initialze first 0th segment, which contains words of program
fn segment0() {
    segments.push()
}