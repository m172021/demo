#[allow(unused_imports)]
use super::*;

#[derive(Clone)]
pub struct Xos_Js_Array {
    pub inner: js_sys::Array,
}

impl Xos_Js_Array {
    pub fn new_jsv(x: &JsValue) -> Result<Xos_Js_Array, Err_Vec_String> {
        if x.is_null() || x.is_undefined() {
            Err(Err_Vec_String::new("Xos_Js_Array :: x is null / undefined".to_string()))?
        }

        let inner = js_sys::Array::from(x);

        if x.is_null() || x.is_undefined() {
            Err(Err_Vec_String::new("Xos_Js_Array :: (x as js_sys::Array) is null / undefined".to_string()))?
        }

        Ok(Xos_Js_Array { inner })
    }

    pub fn to_vec(&self) -> Vec<wasm_bindgen::JsValue> {
        let mut out = vec![];
        let n = self.length();
        for i in 0..n {
            out.push(self.get(i));
        }
        out
    }

    pub fn new_len(n: usize) -> Xos_Js_Array {
        Xos_Js_Array {
            inner: js_sys::Array::new_with_length(n as u32),
        }
    }

    pub fn set(&self, idx: usize, v: JsValue) {
        self.inner.set(idx as u32, v);
    }

    pub fn length(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn get(&self, i: usize) -> wasm_bindgen::JsValue {
        self.inner.get(i as u32)
    }

    pub fn to_jsvalue(&self) -> &wasm_bindgen::JsValue {
        self.inner.deref()
    }

    pub fn into_jsvalue(self) -> wasm_bindgen::JsValue {
        self.inner.into()
    }
}
