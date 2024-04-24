#[allow(unused_imports)]
use super::*;

impl<T: Bintf_T> Bintf_T for Option<T> {
    fn write_to_js(
        &self,
        writer: &mut dyn std::io::Write,

        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<(), Bintf_Err> {
        match self {
            None => 0_u8.write_to_js(writer, _non_transfers, _transfers),
            Some(v) => {
                1_u8.write_to_js(writer, _non_transfers, _transfers)?;
                v.write_to_js(writer, _non_transfers, _transfers)
            }
        }
    }

    fn read_from_js(
        reader: &mut dyn std::io::Read,
        _non_transfers: &mut VecDeque<JsValue>,
        _transfers: &mut VecDeque<JsValue>,
    ) -> Result<Option<T>, Bintf_Err> {
        let x: u8 = Bintf_T::read_from_js(reader, _non_transfers, _transfers)?;
        match x {
            0 => return Ok(None),
            1 => return Ok(Some(<T as Bintf_T>::read_from_js(reader, _non_transfers, _transfers)?)),
            _ => return Err(Bintf_Err::Option),
        }
    }

    fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        match self {
            None => 0_u8.write_to_buf(writer),
            Some(v) => {
                1_u8.write_to_buf(writer)?;
                v.write_to_buf(writer)
            }
        }
    }

    fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Option<T>, Bintf_Err> {
        let x: u8 = Bintf_T::read_from_buf(reader)?;
        match x {
            0 => return Ok(None),
            1 => return Ok(Some(<T as Bintf_T>::read_from_buf(reader)?)),
            _ => return Err(Bintf_Err::Option),
        }
    }
}
