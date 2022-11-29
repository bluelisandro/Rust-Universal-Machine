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

/// Returns true if the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    // If n is between -2^(width - 1) and 2^(width - 1) - 1 return true
    if n >= -(exp_base2(width - 1) as i64) && n <= (exp_base2(width - 1) - 1) as i64 {
        return true;
    }
    return false;
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

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
   // Create a mask for the subset of bits we want
   let mut subset_mask: u64 = 0;

   // Need a mask that has 1s where our value is, so from lsb to lsb + width
   for i in lsb..lsb + width {
       subset_mask = subset_mask | 1 << i;
   }

   // AND to get subset out of word
   let mut value_bits = (word & subset_mask) as i64;

   // Create mask to isolate signed bit from value
   let mut sign_mask: i64 = 0;
   sign_mask = sign_mask | 1 << (lsb + width - 1);

   // If value AND mask is 0, then we know that value does NOT have a 1 at the leftmost bit, therefore it is positive so call getu()
   if value_bits & sign_mask == 0 {
    //    println!("Call getu()");
        return getu(word, width, lsb) as i64; 
   }

   // If value is negative shift the value bits all the way to the right by LSB to get the coded integer value
   value_bits = value_bits >> lsb;

   // If value is negative, we need to XOR it by the same mask we would use in news()
   // Create mask
   let mut neg_mask: u64 = 0;
   for i in 0..(64 - width) {
       neg_mask = neg_mask | 1 << (64 - i - 1);
   }

   return value_bits ^ neg_mask as i64;
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

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitss(value, width) {

        if value < 0 {
            // Create mask
            let mut mask: u64 = 0;
            for i in 0..(64 - width) {
                mask = mask | 1 << (64 - i - 1);
            }

            // Convert signed value to unsigned value using XOR operator with mask
            let unsigned_bits = value as u64 ^ mask;

            // Shift bits
            let new_word: u64 = word | (unsigned_bits << lsb);

            return Some(new_word);
        }
        else {
            return newu(word, width, lsb, value as u64);
        }

    }

   return None;
}
