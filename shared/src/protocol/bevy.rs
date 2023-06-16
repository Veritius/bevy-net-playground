use bevy::prelude::*;
use naia_bevy_shared::{Serde, BitReader, BitWrite, SerdeErr};

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
        let mut v = [[0u8; 4]; 2];

        for i in 0..=4*2 {
            let byte = reader.read_byte();
            if byte.is_err() { return Err(byte.unwrap_err()) }
            let z = i / 4;
            let y = i % 4;
            v[z][y] = byte.unwrap();
        }

        let x = f32::from_ne_bytes(v[0]);
        let y = f32::from_ne_bytes(v[1]);

        let vec = Vec2::new(x, y);

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
        let mut v = [[0u8; 4]; 3];

        for i in 0..=4*3 {
            let byte = reader.read_byte();
            if byte.is_err() { return Err(byte.unwrap_err()) }
            let z = i / 4;
            let y = i % 4;
            v[z][y] = byte.unwrap();
        }

        let x = f32::from_ne_bytes(v[0]);
        let y = f32::from_ne_bytes(v[1]);
        let z = f32::from_ne_bytes(v[2]);

        let vec = Vec3::new(x, y, z);

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
        let mut v = [[0u8; 4]; 4];

        for i in 0..=4*4 {
            let byte = reader.read_byte();
            if byte.is_err() { return Err(byte.unwrap_err()) }
            let z = i / 4;
            let y = i % 4;
            v[z][y] = byte.unwrap();
        }

        let x = f32::from_ne_bytes(v[0]);
        let y = f32::from_ne_bytes(v[1]);
        let z = f32::from_ne_bytes(v[2]);
        let w = f32::from_ne_bytes(v[3]);

        let quat = Quat::from_array([x, y, z, w]);

        return Ok(Self(quat));
    }

    fn bit_length(&self) -> u32 {
        32*4
    }
}