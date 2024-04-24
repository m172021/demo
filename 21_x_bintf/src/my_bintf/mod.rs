#[allow(unused_imports)]
use super::*;

mod my_jscode_arc;
mod my_jscode_array;
mod my_jscode_box;
mod my_jscode_hashmap;
mod my_jscode_hashset;
mod my_jscode_inner_dyn;
mod my_jscode_option;
mod my_jscode_prim;
mod my_jscode_rc;
mod my_jscode_result;
mod my_jscode_string;
mod my_jscode_tup;
mod my_jscode_vec;
mod my_message_port;

pub use my_message_port::*;

pub use my_jscode_inner_dyn::*;

pub trait Bintf_T {
    fn write_to_js(&self, writer: &mut dyn std::io::Write, _non_transfers: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err>;

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized;

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err>;

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized;
}

pub struct Bintf_Base {}

impl Bintf_Base {
    fn js_write_u8_slice(
        writer: &mut dyn std::io::Write,
        obj: &[u8],
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let len = obj.len() as u64;
        len.write_to_js(writer /* abs, */, _non_transfers, _transfers)?;
        Ok(writer.write_all(obj).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn js_read_u8_slice(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<Vec<u8>, Bintf_Err> {
        let n: u64 = Bintf_T::read_from_js(reader /* abs, */, _non_transfers, _transfers)?;
        let mut ans = vec![0; n as usize];
        reader.read_exact(&mut ans).map_err(|_| Bintf_Err::BufReader)?;
        Ok(ans)
    }

    fn bin_write_u8_slice(writer: &mut dyn std::io::Write, obj: &[u8]) -> Result<(), Bintf_Err> {
        let len = obj.len() as u64;
        len.write_to_buf(writer)?;
        Ok(writer.write_all(obj).map_err(|_| Bintf_Err::BufWriter)?)
    }

    fn bin_read_u8_slice(reader: &mut dyn std::io::Read) -> Result<Vec<u8>, Bintf_Err> {
        let n: u64 = Bintf_T::read_from_buf(reader)?;
        let mut ans = vec![0; n as usize];
        reader.read_exact(&mut ans).map_err(|_| Bintf_Err::BufReader)?;
        Ok(ans)
    }
}

pub struct Bintf_Util {}

impl Bintf_Util {
    pub fn my_as_bytes<T: Bintf_T>(obj: &T) -> Result<Vec<u8>, Bintf_Err> {
        let mut v = vec![];
        {
            let mut buf = BufWriter::new(&mut v);
            obj.write_to_buf(&mut buf)?;
        }
        Ok(v)
    }

    pub fn read_from_bytes<T: Bintf_T>(s: &[u8]) -> Result<T, Bintf_Err>
    where
        Self: Sized,
    {
        let mut br = BufReader::new(s);
        T::read_from_buf(&mut br)
    }
}

impl Bintf_Util {
    pub fn conv_xos_raw_msg(tag: u32, obj: &dyn Bintf_T) -> Result<Xos_Raw_Msg, Bintf_Err>
    where
        Self: Sized,
    {
        let mut v = vec![];
        let mut v_ab = VecDeque::new();
        let mut v_vals = VecDeque::new();
        {
            let mut buf = BufWriter::new(&mut v);
            obj.write_to_js(&mut buf, &mut v_ab, &mut v_vals)?;
        }

        let array_buffer = Xos_Js_ArrayBuffer::new_slice(v.as_slice());

        Ok(Xos_Raw_Msg {
            tag,
            rust_part: array_buffer,
            abs: v_ab,
            transfers: v_vals,
        })
    }
}
