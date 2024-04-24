#[allow(unused_imports)]
use super::*;

#[derive(Clone)]
pub struct Xos_Js_ArrayBuffer {
    pub inner: js_sys::ArrayBuffer,
}

impl std::fmt::Debug for Xos_Js_ArrayBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[Xos_Js_ArrayBuffer]]")
    }
}

impl Xos_Js_ArrayBuffer {
    pub fn new_jsv(x: &JsValue) -> Xos_Js_ArrayBuffer {
        let inner = js_sys::ArrayBuffer::from(x.clone());
        assert!(!inner.is_null());
        assert!(!inner.is_undefined());
        Xos_Js_ArrayBuffer { inner }
    }

    pub fn new_slice(s: &[u8]) -> Xos_Js_ArrayBuffer {
        let array_buffer = js_sys::ArrayBuffer::new(s.len() as u32);
        let u8_arr = js_sys::Uint8Array::new(&array_buffer);
        u8_arr.copy_from(s);
        Xos_Js_ArrayBuffer { inner: array_buffer }
    }

    pub fn byte_length(&self) -> usize {
        self.inner.byte_length() as usize
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = vec![0_u8; self.inner.byte_length() as usize];
        let u8_arr = js_sys::Uint8Array::new(&self.inner);
        u8_arr.copy_to(v.as_mut_slice());
        v
    }

    pub fn to_vec_n0(&self, n: usize) -> Vec<u8> {
        let bl = self.inner.byte_length() as usize;
        let mut v = vec![0_u8; bl.max(n)];
        let u8_arr = js_sys::Uint8Array::new(&self.inner);
        u8_arr.copy_to(&mut v.as_mut_slice()[0..bl]);
        v
    }

    pub fn to_vec_part(&self, start: usize, len: usize) -> Vec<u8> {
        let mut v = vec![0_u8; len];

        let u8_arr = js_sys::Uint8Array::new_with_byte_offset_and_length(&self.inner, start as u32, len as u32);

        u8_arr.copy_to(v.as_mut_slice());
        v
    }

    pub fn to_jsvalue(&self) -> &wasm_bindgen::JsValue {
        self.inner.deref()
    }

    pub fn into_jsvalue(self) -> wasm_bindgen::JsValue {
        self.inner.into()
    }
}

impl Bintf_T for JsValue {
    fn write_to_js(&self, _writer: &mut dyn Write, _non_transfers: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err> {
        transfers.push_back(self.into());
        Ok(())
    }

    fn read_from_js(_reader: &mut dyn Read, _non_transfers: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let ab = transfers.pop_front().unwrap();
        Ok(ab)
    }

    fn write_to_buf(&self, _writer: &mut dyn Write) -> Result<(), Bintf_Err> {
        panic!("Xos_Js_ArrayBuffer :: write_to_buf")
    }

    fn read_from_buf(_reader: &mut dyn Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        panic!("Xos_Js_ArrayBuffer :: read_from_but")
    }
}

impl Bintf_T for Xos_Js_ArrayBuffer {
    fn write_to_js(&self, _: &mut dyn std::io::Write, _: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err> {
        transfers.push_back(self.inner.clone().into());
        Ok(())
    }

    fn read_from_js(_: &mut dyn std::io::Read, _: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<Xos_Js_ArrayBuffer, Bintf_Err> {
        let ab = transfers.pop_front().unwrap();
        Ok(Xos_Js_ArrayBuffer::new_jsv(&ab))
    }

    fn write_to_buf(&self, _writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        panic!("Xos_Js_ArrayBuffer :: write_to_buf")
    }

    fn read_from_buf(_reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        panic!("Xos_Js_ArrayBuffer :: read_from_but")
    }
}

pub struct T_Js_ArrayBuffer<T> {
    inner: Xos_Js_ArrayBuffer,
    _t: PhantomData<T>,
}

impl<T> Clone for T_Js_ArrayBuffer<T> {
    fn clone(&self) -> Self {
        T_Js_ArrayBuffer {
            inner: self.inner.clone(),
            _t: Default::default(),
        }
    }
}

impl<T> Debug for T_Js_ArrayBuffer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "T_Js_ArrayBuffer")
    }
}

impl<T> Bintf_T for T_Js_ArrayBuffer<T> {
    fn write_to_js(&self, _writer: &mut dyn Write, _non_transfers: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err> {
        transfers.push_back(self.inner.inner.clone().into());
        Ok(())
    }

    fn read_from_js(_reader: &mut dyn Read, _non_transfers: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let ab = transfers.pop_front().unwrap();
        Ok(T_Js_ArrayBuffer {
            inner: Xos_Js_ArrayBuffer::new_jsv(&ab),
            _t: Default::default(),
        })
    }

    fn write_to_buf(&self, _writer: &mut dyn Write) -> Result<(), Bintf_Err> {
        panic!("T_Js_ArrayBuffer :: write_to_buf")
    }

    fn read_from_buf(_reader: &mut dyn Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        panic!("T_Js_ArrayBuffer :: write_to_buf")
    }
}
