#[allow(unused_imports)]
use super::*;

impl Bintf_T for String {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        Bintf_Base::js_write_u8_slice(writer, self.as_bytes(), _non_transfers, _transfers)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<String, Bintf_Err> {
        let x = Bintf_Base::js_read_u8_slice(reader, _non_transfers, _transfers)?;
        String::from_utf8(x).map_err(|_| Bintf_Err::StringConv)
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        Bintf_Base::bin_write_u8_slice(writer, self.as_bytes())
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<String, Bintf_Err> {
        let x = Bintf_Base::bin_read_u8_slice(reader)?;
        String::from_utf8(x).map_err(|_| Bintf_Err::StringConv)
    }
}
