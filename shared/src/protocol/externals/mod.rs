//! Implementing and improving Naia's Serde trait for external types.

pub mod bevy;

use std::hash::Hash;
use naia_bevy_shared::{BitReader, BitWrite, SerdeErr};

/// Extensions for the [BitWrite] trait. Automatically implemented for all types implementing `BitWrite`.
pub trait BitWriteExt: BitWrite {
    fn write_iter(&mut self, iter: impl Iterator<Item = u32>);
}

impl<T: BitWrite> BitWriteExt for T {
    fn write_iter(&mut self, iter: impl Iterator<Item = u32>) {
        for item in iter {
            self.write_bits(item);
        }
    }
}

/// Extensions for [BitReader].
pub trait BitReaderExt {
    fn read_bits(&mut self) -> Result<u32, SerdeErr>;
}

impl BitReaderExt for BitReader<'_> {
    fn read_bits(&mut self) -> Result<u32, SerdeErr> {
        let mut v = [0u8; 4];
        for i in 0..=3 {
            let byte = self.read_byte();
            if byte.is_err() { return Err(byte.unwrap_err()) }
            v[i] = byte.unwrap();
        }

        Ok(u32::from_ne_bytes(v))
    }
}

/// A wrapper type for Naia's [Serde] trait.
pub struct SerdeWrapper<T>(pub T);

impl<T: Hash> Hash for SerdeWrapper<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: PartialEq> PartialEq for SerdeWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq> Eq for SerdeWrapper<T> {}

impl<T: PartialOrd> PartialOrd for SerdeWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord> Ord for SerdeWrapper<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Clone> Clone for SerdeWrapper<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}