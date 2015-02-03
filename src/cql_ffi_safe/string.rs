extern crate cql_ffi;

use std::str::FromStr;
use std::string::ToString;
use std::slice;

#[derive(Copy,Debug)]
pub struct CassString(pub cql_ffi::CassString);

impl ToString for CassString {
    fn to_string(&self) -> String {unsafe{
        let data = self.0.data as *const u8;
        let slice = slice::from_raw_buf(&data,self.0.length as usize);
        let vec = slice.to_vec();
        String::from_utf8(vec).unwrap()
    }}
}

impl FromStr for CassString {
    fn from_str(string:&str) -> Option<Self> {unsafe{
        let cass_str = cql_ffi::cass_string_init2(string.as_ptr() as *const i8,string.len() as u64);
        Some(CassString(cass_str))
    }}
}
