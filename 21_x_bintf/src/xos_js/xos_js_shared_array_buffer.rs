#[allow(unused_imports)]
use super::*;

/*
#[derive(Clone)]
pub struct Xos_Js_SharedArrayBuffer {
    pub inner: js_sys::SharedArrayBuffer,
}

impl std::fmt::Debug for Xos_Js_SharedArrayBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Xos_Js_SharedArrayBuffer")
    }
}

impl Xos_Js_SharedArrayBuffer {
    pub fn new_jsv(x: &JsValue) -> Xos_Js_SharedArrayBuffer {
        let inner = js_sys::SharedArrayBuffer::from(x.clone());
        assert!(!inner.is_null());
        assert!(!inner.is_undefined());
        Xos_Js_SharedArrayBuffer { inner }
    }

    pub fn to_jsvalue(&self) -> &wasm_bindgen::JsValue {
        self.inner.deref()
    }

    pub fn into_jsvalue(self) -> wasm_bindgen::JsValue {
        self.inner.into()
    }

    pub fn new_slice(s: &[u8]) -> Xos_Js_SharedArrayBuffer {
        let array_buffer = js_sys::SharedArrayBuffer::new(s.len() as u32);
        let u8_arr = js_sys::Uint8Array::new(&array_buffer);
        u8_arr.copy_from(s);
        Xos_Js_SharedArrayBuffer { inner: array_buffer }
    }

    pub fn new_size(n: u32) -> Xos_Js_SharedArrayBuffer {
        let array_buffer = js_sys::SharedArrayBuffer::new(n);
        Xos_Js_SharedArrayBuffer { inner: array_buffer }
    }
}

    fn write_to_js(
        &self,
        _writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        _non_transfers.push_back(self.inner.clone().into());
        Ok(())
    }

    fn read_from_js(
        _reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<Xos_Js_SharedArrayBuffer, Bintf_Err> {
        let ab = _non_transfers.pop_front().unwrap();
        Ok(Xos_Js_SharedArrayBuffer::new_jsv(&ab))
    }

    fn write_to_buf(&self, _writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        damn_it!("")
    }

    fn read_from_buf(_reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        damn_it!("")
    }
}


 */
