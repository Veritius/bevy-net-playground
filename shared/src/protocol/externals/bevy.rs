use bevy::prelude::*;
use naia_bevy_shared::{Serde, BitReader, BitWrite, SerdeErr};
use crate::protocol::externals::BitReaderExt;
use super::SerdeWrapper;

impl Serde for SerdeWrapper<Vec2> {
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

impl Serde for SerdeWrapper<Vec3> {
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

impl Serde for SerdeWrapper<Quat> {
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

        let quat = Quat::from_array(v);

        return Ok(Self(quat));
    }

    fn bit_length(&self) -> u32 {
        32*4
    }
}

impl Serde for SerdeWrapper<Color> {
    fn ser(&self, writer: &mut dyn BitWrite) {
        let v = self.0.as_linear_rgba_f32();
        for x in v {
            writer.write_bits(x.to_bits());
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

        let color = Color::rgba(v[0], v[1], v[2], v[3]);

        return Ok(Self(color));
    }

    fn bit_length(&self) -> u32 {
        32*4
    }
}

impl Serde for SerdeWrapper<Transform> {
    fn ser(&self, writer: &mut dyn BitWrite) {
        let pos = SerdeWrapper(self.0.translation);
        let rot = SerdeWrapper(self.0.rotation);
        let sca = SerdeWrapper(self.0.scale);

        pos.ser(writer);
        rot.ser(writer);
        sca.ser(writer);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        let pos = SerdeWrapper::<Vec3>::de(reader);
        let rot = SerdeWrapper::<Quat>::de(reader);
        let sca = SerdeWrapper::<Vec3>::de(reader);
        
        if pos.is_err() | rot.is_err() | sca.is_err() {
            return Err(SerdeErr)
        }

        Ok(SerdeWrapper(Transform {
            translation: pos.unwrap().0,
            rotation: rot.unwrap().0,
            scale: sca.unwrap().0
        }))
    }

    fn bit_length(&self) -> u32 {
        32*(6+4)
    }
}