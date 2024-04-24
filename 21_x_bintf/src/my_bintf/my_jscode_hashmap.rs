#[allow(unused_imports)]
use super::*;

impl<K: Bintf_T + Hash + Eq, V: Bintf_T> Bintf_T for HashMap<K, V> {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let len = self.len() as u64;
        len.write_to_js(writer, _non_transfers, _transfers)?;
        for x in self.iter() {
            x.0.write_to_js(writer, _non_transfers, _transfers)?;
            x.1.write_to_js(writer, _non_transfers, _transfers)?;
        }
        Ok(())
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<HashMap<K, V>, Bintf_Err> {
        let len = <u64 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)? as usize;
        let mut ans = HashMap::new();
        for _i in 0..len {
            let k = <K as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
            let v = <V as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
            ans.insert(k, v);
        }
        Ok(ans)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let len = self.len() as u64;
        len.write_to_buf(writer)?;
        for x in self.iter() {
            x.0.write_to_buf(writer)?;
            x.1.write_to_buf(writer)?;
        }
        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<HashMap<K, V>, Bintf_Err> {
        let len = <u64 as Bintf_T>::read_from_buf(reader)? as usize;
        let mut ans = HashMap::new();
        for _i in 0..len {
            let k = <K as Bintf_T>::read_from_buf(reader)?;
            let v = <V as Bintf_T>::read_from_buf(reader)?;
            ans.insert(k, v);
        }
        Ok(ans)
    }
}
