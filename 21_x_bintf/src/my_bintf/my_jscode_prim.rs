#[allow(unused_imports)]
use super::*;

/*
use std::{
    atomic,
    atomic::{AtomicU16, AtomicU8},
};

 */

impl Bintf_T for bool {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let x = if *self { 1_u8 } else { 0_u8 };
        Ok(writer.write_u8(x).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<bool, Bintf_Err> {
        let x: u8 = <u8 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
        Ok(x == 1)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let x = if *self { 1_u8 } else { 0_u8 };
        Ok(writer.write_u8(x).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<bool, Bintf_Err> {
        let x: u8 = <u8 as Bintf_T>::read_from_buf(reader)?;
        Ok(x == 1)
    }
}

// 8 bits

impl Bintf_T for i8 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_i8(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<i8, Bintf_Err> {
        reader.read_i8().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_i8(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<i8, Bintf_Err> {
        reader.read_i8().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for u8 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_u8(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<u8, Bintf_Err> {
        reader.read_u8().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_u8(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<u8, Bintf_Err> {
        reader.read_u8().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for AtomicU8 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_u8(self.load(atomic::Ordering::Relaxed)).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        Ok(AtomicU8::new(reader.read_u8().map_err(|_| Bintf_Err::BufReader)?))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_u8(self.load(atomic::Ordering::Relaxed)).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        Ok(AtomicU8::new(reader.read_u8().map_err(|_| Bintf_Err::BufReader)?))
    }
}

// 16 bits

impl Bintf_T for i16 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_i16::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<i16, Bintf_Err> {
        reader.read_i16::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_i16::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<i16, Bintf_Err> {
        reader.read_i16::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for u16 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_u16::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<u16, Bintf_Err> {
        reader.read_u16::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_u16::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<u16, Bintf_Err> {
        reader.read_u16::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for AtomicU16 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer
            .write_u16::<LittleEndian>(self.load(atomic::Ordering::Relaxed))
            .map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        Ok(AtomicU16::new(reader.read_u16::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)?))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer
            .write_u16::<LittleEndian>(self.load(atomic::Ordering::Relaxed))
            .map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        Ok(AtomicU16::new(reader.read_u16::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)?))
    }
}

// 32 bits

impl Bintf_T for i32 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_i32::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<i32, Bintf_Err> {
        reader.read_i32::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_i32::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<i32, Bintf_Err> {
        reader.read_i32::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for f32 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_f32::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<f32, Bintf_Err> {
        reader.read_f32::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_f32::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<f32, Bintf_Err> {
        reader.read_f32::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for u32 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_u32::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _i_transfers: &mut VecDeque<JsValue>) -> Result<u32, Bintf_Err> {
        reader.read_u32::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_u32::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<u32, Bintf_Err> {
        reader.read_u32::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}

// 64 bits

impl Bintf_T for i64 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_i64::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<i64, Bintf_Err> {
        reader.read_i64::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_i64::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<i64, Bintf_Err> {
        reader.read_i64::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for u64 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_u64::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<u64, Bintf_Err> {
        reader.read_u64::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_u64::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<u64, Bintf_Err> {
        reader.read_u64::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for f64 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_f64::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<f64, Bintf_Err> {
        reader.read_f64::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_f64::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<f64, Bintf_Err> {
        reader.read_f64::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}

impl Bintf_T for u128 {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Ok(writer.write_u128::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<u128, Bintf_Err> {
        reader.read_u128::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Ok(writer.write_u128::<LittleEndian>(*self).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<u128, Bintf_Err> {
        reader.read_u128::<LittleEndian>().map_err(|_| Bintf_Err::BufReader)
    }
}
