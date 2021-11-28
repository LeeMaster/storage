//! # Bits Vector
//!
//! Plain Bits Vector [Wiki](https://en.wikipedia.org/wiki/Bit_array)
//!
//! All the operations will be bootstraped from the simple bits operations, e.g. And, Or, Not,
//! Xor. Because of the long bits vector composed from many underlying block, in common situation  
//! the Block is standard type in PL, such as u8, u16, u32, usize. Since the bits vector is an
//! array structure.
//!
//! ## How to implement it ？
//!
//! TODO: complete this document

use std::hash;
use std::ops::*;
use std::vec;

/// Bits vector underlying abstraction of the bits block
pub trait Block:
    Copy
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Not<Output = Self>
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Rem<Self, Output = Self>
    + Eq
    + Ord
    + hash::Hash
{
    /// identify how many bits in this block
    /// every block should be aligned to 8 bit ?
    fn bits() -> usize;

    /// identify how many bytes in this block
    #[inline]
    fn bytes() -> usize {
        Self::bits() / 8
    }

    /// create all zero bits instance
    fn zero() -> Self;

    /// create all one bits instance
    fn one() -> Self;
}

// helpr for impl the Block interface
macro_rules! block_impl {
    ($(($t: ident, $size: expr)),*) => ($(
        impl Block for $t {
            #[inline]
            fn bits() -> usize { $size }
            #[inline]
            fn one() -> Self { 1 }
            #[inline]
            fn zero() -> Self{ 0 }
        }
    )*)

}

// impl the Block interface for primary types
block_impl! {
    (u8, 8),
    (u16, 16),
    (u32, 32),
    (u64, 64),
    (usize, core::mem::size_of::<usize>() * 8)
}

// should have another macro to impl the Block trait to auto type u8 u16 or usize

/// calculate the blocks numbers of bits number
#[inline]
fn calculate_blocks<B: Block>(bits: usize) -> usize {
    if bits % B::bits() == 0 {
        bits / B::bits()
    } else {
        bits / B::bits() + 1
    }
}

/// calculate the final block of a vector
#[inline]
fn mask_block<B: Block>(bits: usize) -> B {
    // why the final block is all left bitwase one block like 00011111
    // FIXME: if all zero not work ?
    (!B::zero()) >> ((B::bits() - bits % B::bits()) % B::bits())
}

pub struct Vector<B = u32> {
    storage: vec::Vec<B>,
    bits: usize,
}

impl<B: Block> Index<usize> for Vector<B> {
    type Output = bool;

    /// index method for vector to access specific position bit
    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &false
    }
}

pub struct Iter {}

pub struct IntoIter {}

#[cfg(test)]
mod tests {}
