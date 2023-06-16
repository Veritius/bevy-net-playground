use bevy::prelude::*;
use naia_bevy_shared::{Serde, BitReader, BitWrite, SerdeErr};

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

pub struct ExternalWrapper<T>(pub T);

impl<T: PartialEq> PartialEq for ExternalWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Clone> Clone for ExternalWrapper<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Serde for ExternalWrapper<Vec2> {
    fn ser(&self, writer: &mut dyn BitWrite) {
        let v = self.0;
        writer.write_bits(v.x.to_bits());
        writer.write_bits(v.y.to_bits());
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        const LENGTH: usize = 2;

        let mut v = [0.0f32; LENGTH];

        for i in 0..=LENGTH {
            let bits = reader.read_bits();
            if bits.is_err() { return Err(bits.unwrap_err()) }
            v[i] = f32::from_bits(bits.unwrap());
        }

        let vec = Vec2::new(v[0], v[1]);

        return Ok(Self(vec));
    }

    fn bit_length(&self) -> u32 {
        32*2
    }
}

impl Serde for ExternalWrapper<Vec3> {
    fn ser(&self, writer: &mut dyn BitWrite) {
        let v = self.0;
        writer.write_bits(v.x.to_bits());
        writer.write_bits(v.y.to_bits());
        writer.write_bits(v.z.to_bits());
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        const LENGTH: usize = 3;

        let mut v = [0.0f32; LENGTH];

        for i in 0..=LENGTH {
            let bits = reader.read_bits();
            if bits.is_err() { return Err(bits.unwrap_err()) }
            v[i] = f32::from_bits(bits.unwrap());
        }

        let vec = Vec3::new(v[0], v[1], v[2]);

        return Ok(Self(vec));
    }

    fn bit_length(&self) -> u32 {
        32*3
    }
}

impl Serde for ExternalWrapper<Quat> {
    fn ser(&self, writer: &mut dyn BitWrite) {
        let v = self.0.to_array();
        for i in 0..=3 {
            writer.write_bits(v[i].to_bits());
        }
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        const LENGTH: usize = 4;

        let mut v = [0.0f32; LENGTH];

        for i in 0..=LENGTH {
            let bits = reader.read_bits();
            if bits.is_err() { return Err(bits.unwrap_err()) }
            v[i] = f32::from_bits(bits.unwrap());
        }

        let vec = Quat::from_array(v);

        return Ok(Self(vec));
    }

    fn bit_length(&self) -> u32 {
        32*4
    }
}