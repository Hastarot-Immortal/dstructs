use std::f32::consts::LN_2;
use murmur2::{murmur2, murmur64a};
use zerocopy::{Immutable, IntoBytes};

use crate::has_length::HasLength;

pub struct BloomFilter<T, H: HasherWithSeed = MurmurHash, C: BloomFilterContainer<T> = Vec<T>>
where
    T: Immutable + IntoBytes + PartialEq
{
    capacity: usize,
    mask: Mask,
    hasher: H,
    container: C,
    hasher_number: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> BloomFilter<T, MurmurHash, Vec<T>>
where
    T: Immutable + IntoBytes + PartialEq,
{
    pub fn with_capacity(capacity: usize) -> Self {
        let (size_mask, hasher_number) = Self::fast_mask_and_hasher_size(capacity, Self::ABS_ERROR_RATE_LN);
        
        Self { 
            capacity,
            mask: Mask::with_size(size_mask), 
            hasher: MurmurHash, 
            container: Vec::new(), 
            _marker: std::marker::PhantomData, 
            hasher_number,
        }
    }

    pub fn with_error_rate(capacity: usize, error_rate: f32) -> Self {
        let (size_mask, hasher_number) = Self::mask_and_hasher_size(capacity, error_rate);

        Self {
            capacity,
            mask: Mask::with_size(size_mask),
            hasher: MurmurHash,
            container: Vec::new(),
            hasher_number,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, H, C> BloomFilter<T, H, C>
where
    T: Immutable + IntoBytes + PartialEq,
    H: HasherWithSeed,
    C: BloomFilterContainer<T>
{
    const ABS_ERROR_RATE_LN: f32 = 2.9957323; // |ln(0.05)|

    fn fast_mask_and_hasher_size(capacity: usize, error_rate: f32) -> (usize, usize) {
        let size_mask = (error_rate * (capacity as f32)) / (LN_2 * LN_2);
        let k: usize = (error_rate / LN_2) as usize;

        (size_mask as usize, if k > 0 { k } else { 1 })
    }

    fn mask_and_hasher_size(capacity: usize, error_rate: f32) -> (usize, usize) {
        Self::fast_mask_and_hasher_size(capacity, error_rate.ln().abs())
    }

    pub fn with_hasher(capacity: usize, hasher: H) -> Self {
        let (size_mask, hasher_number) = Self::fast_mask_and_hasher_size(capacity, Self::ABS_ERROR_RATE_LN);

        Self { 
            capacity,
            mask: Mask::with_size(size_mask), 
            hasher, 
            container: C::new(), 
            hasher_number, 
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_all(capacity: usize, error_rate: f32, hasher: H) -> Self {
        let (size_mask, hasher_number) = Self::mask_and_hasher_size(capacity, error_rate);

        Self {
            capacity,
            mask: Mask::with_size(size_mask),
            hasher,
            container: C::new(),
            hasher_number,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        for k in 0..self.hasher_number {
            if !self.mask.check(self.hasher.hash(value.as_bytes(), k) % self.mask.len()) {
                return false;
            }
        }
        self.container.contains(value)
    }

    pub fn insert(&mut self, value: T) {
        if self.is_empty() {
            for k in 0..self.hasher_number {
                self.mask.update(self.hasher.hash(value.as_bytes(), k) % self.mask.len());
            }
            self.container.insert(value);
        } else if (self.len() < self.capacity) && (!self.contains(&value) || !self.container.contains(&value)){
            for k in 0..self.hasher_number {
                self.mask.update(self.hasher.hash(value.as_bytes(), k) % self.mask.len());
            }
            self.container.insert(value);
        } 
    }
}

impl<T, H, C> HasLength for BloomFilter<T, H, C>
where 
    T: Immutable + IntoBytes + PartialEq,
    H: HasherWithSeed,
    C: BloomFilterContainer<T>
{
    fn len(&self) -> usize {
        self.container.len()
    }
}

pub trait BloomFilterContainer<T>: HasLength {
    fn new() -> Self;
    fn contains(&self, value: &T) -> bool;
    fn insert(&mut self, value: T);
}

impl<T> HasLength for Vec<T> {
    fn len(&self) -> usize {
        Vec::<T>::len(&self)
    }
}

impl<T> BloomFilterContainer<T> for Vec<T> 
where T: PartialEq{
    fn new() -> Self {
        vec![]
    }

    fn contains(&self, value: &T) -> bool {
       self.as_slice().contains(value)
    }

    fn insert(&mut self, value: T) {
        self.push(value);
    }
}

pub trait HasherWithSeed {
    fn hash(&self, data: &[u8], seed: usize) -> usize;
}

pub struct MurmurHash;

impl HasherWithSeed for MurmurHash {
    fn hash(&self, data: &[u8], seed: usize) -> usize {
        if cfg!(target_pointer_width = "32") {
            murmur2(data, seed as u32) as usize
        } else {
            murmur64a(data, seed as u64) as usize
        }
    }
}

struct Mask {
    bytes: Vec<u8>,
}

impl Mask {
    fn with_size(size: usize) -> Self {
        Self {
            bytes: vec![0; (size >> 3) + 1]
        }
    }

    fn len(&self) -> usize {
        self.bytes.len() << 3
    }

    fn update(&mut self, bit_number: usize) {
        let byte_number = bit_number >> 3;
        let bit_number_in_byte = 0b1000_0000 >> (bit_number - (byte_number << 3));
        self.bytes[byte_number] |= bit_number_in_byte;
    }

    fn check(&self, bit_number: usize) -> bool {
        let byte_number = bit_number >> 3;
        let bit_number_in_byte = 0b1000_0000 >> (bit_number - (byte_number << 3));
        (self.bytes[byte_number] & bit_number_in_byte).count_ones() == 1
    }
}
