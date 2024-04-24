#[allow(unused_imports)]
use super::*;

pub trait Bintf_Dyn {
    fn from_xos_raw_msg(xos_raw_msg: &Xos_Raw_Msg) -> Result<Self, Bintf_Err>
    where
        Self: Sized;
}

impl<T: Bintf_T> Bintf_Dyn for T {
    fn from_xos_raw_msg(xos_raw_msg: &Xos_Raw_Msg) -> Result<Self, Bintf_Err>
    where
        Self: Sized,
    {
        let v = xos_raw_msg.rust_part.to_vec();
        let mut v_ab = xos_raw_msg.abs.clone();
        let mut v_vals = xos_raw_msg.transfers.clone();

        let ans = {
            let mut buf = BufReader::new(v.as_slice());
            Self::read_from_js(&mut buf, &mut v_ab, &mut v_vals)?
        };

        Ok(ans)
    }
}
