#[allow(unused_imports)]
use super::*;

impl<K: Bintf_T + Hash + Eq> Bintf_T for HashSet<K> {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        let len = self.len() as u64;
        len.write_to_js(writer, _non_transfers, _transfers)?;
        for x in self.iter() {
            x.write_to_js(writer, _non_transfers, _transfers)?;
        }
        Ok(())
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<HashSet<K>, Bintf_Err> {
        let len = <u64 as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)? as usize;
        let mut ans = HashSet::new();
        for _i in 0..len {
            let k = <K as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?;
            ans.insert(k);
        }
        Ok(ans)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        let len = self.len() as u64;
        len.write_to_buf(writer)?;
        for x in self.iter() {
            x.write_to_buf(writer)?;
        }
        Ok(())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<HashSet<K>, Bintf_Err> {
        let len = <u64 as Bintf_T>::read_from_buf(reader)? as usize;
        let mut ans = HashSet::new();
        for _i in 0..len {
            let k = <K as Bintf_T>::read_from_buf(reader)?;
            ans.insert(k);
        }
        Ok(ans)
    }
}
