use std::{
    cmp::max,
    ops::{Add, Div, Sub},
};

use crate::consts::WORD_BYTES;
use primitive_types::U256;

pub struct Memory {
    data: Vec<u8>,
    len_words: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len_words: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.len_words * WORD_BYTES
    }

    fn resize(&mut self, length: usize) {
        self.data.resize(length, 0);
        self.len_words = max(self.len_words, ceil_divide(length, WORD_BYTES)).into();
    }

    // memory′[offset . . . (offset + 31)] ≡ value
    // num_words_in_mem′≡max(num_words_in_mem, ceil( (offset+32)÷32 ) )
    pub fn set(&mut self, byte_offset: usize, value: U256, target_size: usize) -> Result<(), ()> {
        self.resize(byte_offset + target_size);

        for i in 0..target_size {
            let byte = value.byte(target_size - 1 - i);
            self.data[byte_offset + i] = byte;
        }

        Ok(())
    }

    pub fn get(&mut self, byte_offset: usize) -> &[u8] {
        let end_index = byte_offset + WORD_BYTES;
        if end_index >= self.len_words {
            self.resize(end_index);
        }
        &self.data[byte_offset..end_index]
    }
}

// TODO: clean up this mess 🤦‍♂️
fn ceil_divide<T: Int>(a: T, b: T) -> T {
    (a + b - T::one()) / b
}

trait Int: Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + PartialEq + Copy {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Int for U256 {
    fn zero() -> Self {
        U256::zero()
    }
    fn one() -> Self {
        U256::one()
    }
}

impl Int for usize {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Int for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
