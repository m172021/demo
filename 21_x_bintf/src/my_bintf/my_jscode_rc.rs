#[allow(unused_imports)]
use super::*;

impl<T: Bintf_T> Bintf_T for Rc<T> {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        self.as_ref().write_to_js(writer, _non_transfers, _transfers)
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Rc<T>, Bintf_Err> {
        Ok(Rc::new(T::read_from_js(reader, _non_transfers, _transfers)?))
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        self.as_ref().write_to_buf(writer)
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Rc<T>, Bintf_Err> {
        Ok(Rc::new(T::read_from_buf(reader)?))
    }
}
