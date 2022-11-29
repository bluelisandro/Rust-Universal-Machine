#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Computes 2 to the nth power iteratively, and returns product
pub fn exp_base2(n: u64) -> u64 {
    let mut product = 1;
    for _ in 0..n {
        product = product * 2;
    }
    return product;
}

/// Returns true if the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
// This function is going to be given an integer that was converted from a float
pub fn fitsu(n: u64, width: u64) -> bool {
    if n <= exp_base2(width) {
        return true;
    }
    return false;
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    // To get a subset of bits from a binary string, we need to use bitmasking
    // In this case, we want to create a mask that spans for the given width, ending at the LSB

    // Create mask of size width, starting at LSB
    let mut mask: u64 = 0;
    for i in 0..width {
        mask = mask | 1 << (lsb + i);
    }

    // Then to get the subset of bits that we want from the word, perform an XOR operation
    let mut value: u64 = word & mask;

    // Then, shift the value bits to the right by LSB to get the coded integer value
    value = value >> lsb;

    return value;
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if fitsu(value, width) {
        // Add value to word, then shift by lsb bits
        return Some(word | (value << lsb));
    }

   return None;
}