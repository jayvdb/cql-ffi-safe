extern crate cql_ffi;

use std::io::net::ip::IpAddr;
use std::io::net::ip::Ipv4Addr;
use std::io::net::ip::Ipv6Addr;

#[derive(Copy)]
pub struct CassInet {
    pub inet:cql_ffi::CassInet
}

pub fn init(addr:IpAddr) -> CassInet {unsafe{
    match addr {
        Ipv4Addr(oct1,oct2,oct3,oct4) => {
            let mut v:Vec<u8> = Vec::with_capacity(4);
            v.push(oct1);v.push(oct2);v.push(oct3);v.push(oct4);
            CassInet{inet:cql_ffi::cass_inet_init_v4(v.as_ptr())}
        },
        Ipv6Addr(seg1,seg2,seg3,seg4,seg5,seg6,seg7,seg8) => {
            let mut v:Vec<u16> = Vec::with_capacity(8);
            v.push(seg1);v.push(seg2);v.push(seg3);v.push(seg4);
            v.push(seg5);v.push(seg6);v.push(seg7);v.push(seg8);
            panic!("ip46 not yet supported")
            //CassInet{inet:cql_ffi::cass_inet_init_v6(v.as_ptr())}
        }
    }
}}