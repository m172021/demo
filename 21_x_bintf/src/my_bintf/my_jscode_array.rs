#[allow(unused_imports)]
use super::*;
use std::mem;
use std::mem::MaybeUninit;

// 8 bit

unsafe fn foo<const N: usize, T: Sized>(x: [MaybeUninit<T>; N]) -> [T; N] {
    let mut x = x;
    let ptr = &mut x as *mut _ as *mut [T; N];
    let res = unsafe { ptr.read() };
    core::mem::forget(x);
    res
}

impl<const N: usize, T: Bintf_T + Sized> Bintf_T for [T; N] {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        for i in 0..N {
            T::write_to_js(&self[i], writer, _non_transfers, _transfers)?;
        }
        Ok(())
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            let t = T::read_from_js(reader, _non_transfers, _transfers)?;
            out[i].write(t);
        }
        Ok(unsafe { foo(out) })
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        for i in 0..N {
            T::write_to_buf(&self[i], writer)?;
        }
        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            let t = T::read_from_buf(reader)?;
            out[i].write(t);
        }
        Ok(unsafe { foo(out) })
    }
}

/*
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        writer.write_all(&self[..]).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_u8; N];
        reader.read_exact(&mut out[..]).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        writer.write_all(&self[..]).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_u8; N];
        reader.read_exact(&mut out[..]).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}
*/

/*
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_i8; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_i8; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}

// 16 bit

    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 2) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_i16; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 2 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 2) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_i16; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 2 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}

    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 2) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_u16; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 2 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 2) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_u16; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 2 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}

// 32 bit

    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 4) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_i32; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 4 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 4) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_i32; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 4 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}

    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 4) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_u32; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 4 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 4) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_u32; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 4 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}

    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 4) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_f32; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 4 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 4) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_f32; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 4 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}

// 64 bit

    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 8) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_i64; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 8 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 8) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_i64; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 8 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}

    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 8) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_u64; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 8 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 8) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_u64; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 8 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}

    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 8) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_f64; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 8 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let write_u8 = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, N * 8) };
        writer.write_all(write_u8).map_err(|_| Bintf_Err::BufWriter)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let mut out = [0_f64; N];
        let read_u8 = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, 8 * N) };
        reader.read_exact(read_u8).map_err(|_| Bintf_Err::BufReader)?;
        Ok(out)
    }
}


 */
