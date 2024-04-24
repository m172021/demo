#[allow(unused_imports)]
use super::*;

/*
#[derive(Clone, Debug)]
pub struct Xos_Js_ArrayBuffer_Vec<T: Copy> {
    pub len: u32,
    pub inner: Xos_Js_ArrayBuffer,
    pub _t: PhantomData<T>,
}

fn conv_bytes<T>(v: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * std::mem::size_of::<T>()) }
}

impl<T: Copy> Xos_Js_ArrayBuffer_Vec<T> {
    pub fn new_slice(x: &[T]) -> Xos_Js_ArrayBuffer_Vec<T> {
        Xos_Js_ArrayBuffer_Vec {
            len: x.len() as u32,
            inner: Xos_Js_ArrayBuffer::new_slice(conv_bytes(x)),
            _t: Default::default(),
        }
    }
}

    fn write_to_js(&self, writer: &mut dyn std::io::Write, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err>
    where
        Self: Sized,
    {
        self.len.write_to_js(writer, _non_transfers, _transfers)?;
        self.inner.write_to_js(writer, _non_transfers, _transfers)?;
        Ok(())
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let len = u32::read_from_js(reader, _non_transfers, _transfers)?;
        let inner = Xos_Js_ArrayBuffer::read_from_js(reader, _non_transfers, _transfers)?;

        Ok(Xos_Js_ArrayBuffer_Vec {
            len,
            inner,
            _t: Default::default(),
        })
    }

    fn write_to_buf(&self, _writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        damn_it!("Xos_Js_ArrayBuffer_Vec can not write to bin");
    }

    fn read_from_buf(_reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        damn_it!("Xos_Js_ArrayBuffer_Vec can not read from ");
    }
}


 */
