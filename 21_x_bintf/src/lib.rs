#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use x_include_base::*;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use wasm_bindgen::JsValue;
use x_macros::*;

mod my_bintf;
mod webrtc;
mod xos_js;
mod xos_raw_msg;

pub use my_bintf::*;
pub use webrtc::*;
pub use xos_js::*;
pub use xos_raw_msg::*;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[derive(Debug)]
pub enum Bintf_Err {
    BufWriter,
    Illegal_Enum,
    Option,
    StringConv,
    BufReader,
    Unknown,
    Other(String),
}

impl std::error::Error for Bintf_Err {}

impl std::fmt::Display for Bintf_Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Err_Vec_String {
    data: Vec<String>,
}

impl Debug for Err_Vec_String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "").unwrap();
        for x in self.data.iter() {
            writeln!(f, "{}", x).unwrap();
        }
        writeln!(f, "").unwrap();
        Ok(())
    }
}

impl Err_Vec_String {
    pub fn new(x: String) -> Err_Vec_String {
        Err_Vec_String { data: vec![x] }
    }

    pub fn push(mut self, s: String) -> Err_Vec_String {
        self.data.push(s);
        self
    }
}
