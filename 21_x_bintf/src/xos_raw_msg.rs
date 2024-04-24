use super::*;

pub struct Xos_Raw_Msg {
    pub tag: u32,
    pub rust_part: Xos_Js_ArrayBuffer,
    pub abs: VecDeque<JsValue>,
    pub transfers: VecDeque<JsValue>,
}

impl Bintf_T for Xos_Raw_Msg {
    fn write_to_js(&self, writer: &mut dyn std::io::Write, _non_transfers: &mut VecDeque<JsValue>, transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err>
    where
        Self: Sized,
    {
        writer.write_u32::<LittleEndian>(self.tag as u32).unwrap();
        writer.write_u32::<LittleEndian>(self.abs.len() as u32).unwrap();
        writer.write_u32::<LittleEndian>(self.transfers.len() as u32).unwrap();

        transfers.push_back(self.rust_part.clone().inner.into());

        // _non_transfers.push_back(self.rust_part.clone().into());
        for x in self.abs.iter() {
            _non_transfers.push_back(x.clone());
        }

        for x in self.transfers.iter() {
            transfers.push_back(x.clone());
        }

        Ok(())
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        // abs: &mut VecDeque<JsValue>,
        _non_fransfers: &mut VecDeque<JsValue>,
        transfers: &mut VecDeque<JsValue>,
    ) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let tag = reader.read_u32::<LittleEndian>().unwrap(); // un(err!("could not read tag"));
        let n_abs = reader.read_u32::<LittleEndian>().unwrap(); // un(err!("could not read abs.len"));
        let n_transfers = reader.read_u32::<LittleEndian>().unwrap(); // un(err!("could not read transfers.len()"));

        let rust_part = Xos_Js_ArrayBuffer::new_jsv(&transfers.pop_front().unwrap());

        let mut o_abs = VecDeque::new();
        for _i in 0..(n_abs as usize) {
            o_abs.push_back(_non_fransfers.pop_front().unwrap());
        }

        let mut o_transfers = VecDeque::new();
        for _i in 0..(n_transfers as usize) {
            o_transfers.push_back(transfers.pop_front().unwrap())
        }
        Ok(Xos_Raw_Msg {
            tag,
            rust_part,
            abs: o_abs,
            transfers: o_transfers,
        })
    }

    fn write_to_buf(&self, _writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        todo!("")
    }

    fn read_from_buf(_reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        todo!("")
    }
}

impl Xos_Raw_Msg {
    pub fn to_typed<T: Bintf_T>(&self) -> Result<T, Bintf_Err> {
        T::from_xos_raw_msg(self)
    }

    pub fn get_transfers(&self) -> JsValue {
        let transfers = js_sys::Array::new_with_length(self.transfers.len() as u32);
        for (i, x) in self.transfers.iter().enumerate() {
            js_sys::Array::set(&transfers, i as u32, x.clone())
        }
        transfers.into()
    }

    pub fn to_js(&self) -> wasm_bindgen::JsValue {
        let out = js_sys::Object::new();

        let abs = js_sys::Array::new_with_length(self.abs.len() as u32);
        for (i, x) in self.abs.iter().enumerate() {
            js_sys::Array::set(&abs, i as u32, x.clone().into())
        }

        let transfers = js_sys::Array::new_with_length(self.transfers.len() as u32);
        for (i, x) in self.transfers.iter().enumerate() {
            js_sys::Array::set(&transfers, i as u32, x.clone())
        }

        js_sys::Reflect::set(&out, &"rust_part".into(), &self.rust_part.inner.clone().into()).unwrap();
        js_sys::Reflect::set(&out, &"non_transfers".into(), &abs.into()).unwrap();
        js_sys::Reflect::set(&out, &"transfers".into(), &transfers.into()).unwrap();
        js_sys::Reflect::set(&out, &"tag".into(), &self.tag.into()).unwrap();

        out.into()
    }

    pub fn from_js(v: wasm_bindgen::JsValue) -> Result<Xos_Raw_Msg, Err_Vec_String> {
        let rust_part =
            js_sys::Reflect::get(&v, &"rust_part".into()).map_err(|_x| Err_Vec_String::new("Xos_Raw_Msg :: from_js failed to read .rust_part".to_string()))?;
        let transfers =
            js_sys::Reflect::get(&v, &"transfers".into()).map_err(|_x| Err_Vec_String::new("Xos_Raw_Msg :: from_js failed to read .transfers".to_string()))?;
        let transfers = Xos_Js_Array::new_jsv(&transfers)?.to_vec();
        let non_transfers = js_sys::Reflect::get(&v, &"non_transfers".into())
            .map_err(|_x| Err_Vec_String::new("Xos_Raw_Msg :: from_js failed to read .non_transfers".to_string()))?;
        let tag = js_sys::Reflect::get(&v, &"tag".into()).map_err(|_x| Err_Vec_String::new("Xos_Raw_Msg :: from_js failed to read .tag".to_string()))?;
        let non_transfers = Xos_Js_Array::new_jsv(&non_transfers)?.to_vec();

        Ok(Xos_Raw_Msg {
            tag: tag.as_f64().unwrap() as u32,
            rust_part: Xos_Js_ArrayBuffer::new_jsv(&rust_part),
            abs: non_transfers.into(),
            transfers: transfers.into(),
        })
    }
}

impl std::fmt::Debug for Xos_Raw_Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Xos_Raw_Msg")
    }
}
