#[allow(unused_imports)]
use super::*;

impl<T: Bintf_T, E: Bintf_T> Bintf_T for Result<T, E> {
    fn write_to_js(&self, writer: &mut dyn std::io::Write, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<(), Bintf_Err>
    where
        Self: Sized,
    {
        match self {
            Ok(t) => {
                writer.write_u8(0).unwrap();
                T::write_to_js(t, writer, _non_transfers, _transfers)
            }
            Err(e) => {
                writer.write_u8(1).unwrap();
                E::write_to_js(e, writer, _non_transfers, _transfers)
            }
        }
    }

    fn read_from_js(reader: &mut dyn std::io::Read, _non_transfers: &mut VecDeque<JsValue>, _transfers: &mut VecDeque<JsValue>) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let s = reader.read_u8().map_err(|_| Bintf_Err::Unknown)?;
        let t = match s {
            0 => Ok(T::read_from_js(reader, _non_transfers, _transfers)?),
            1 => Err(E::read_from_js(reader, _non_transfers, _transfers)?),
            _ => Err(Bintf_Err::Unknown)?,
        };
        Ok(t)
    }

    fn write_to_buf(&self, _writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
        panic!("Result :: write_to_buf")
    }

    fn read_from_buf(_reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        panic!("Result :: read_from_buf")
    }
}
