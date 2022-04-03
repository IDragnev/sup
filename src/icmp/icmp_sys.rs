#![allow(non_snake_case)]

use crate::{
    ipv4,
    lib_loader::Library,
};
use std::ffi::c_void;

pub type Handle = *const c_void;

macro_rules! bind {
    ( $(fn $name:ident($($arg:ident: $type:ty),*) -> $ret:ty;)* ) => {
        struct Functions {
            $(pub $name: extern "stdcall" fn ($($arg: $type),*) -> $ret),*
        }

        static FUNCTIONS: once_cell::sync::Lazy<Functions> =
            once_cell::sync::Lazy::new(|| {
                let lib = Library::new("IPHLPAPI.dll").unwrap();
                Functions {
                    $($name: lib.get_proc(stringify!($name)).unwrap()),*
                }
            });

        $(
            #[inline(always)]
            pub fn $name($($arg: $type),*) -> $ret {
                (FUNCTIONS.$name)($($arg),*)
            }
        )*
    };
}

bind! {
fn IcmpCreateFile() -> Handle;
fn IcmpCloseHandle(handle: Handle) -> ();
fn IcmpSendEcho(
    handle: Handle,
    dest: ipv4::Addr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32) -> u32;
}

#[repr(C)]
#[derive(Debug)]
pub struct IpOptionInformation {
    pub ttl: u8,
    pub tos: u8,
    pub flags: u8,
    pub options_size: u8,
    pub options_data: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct IcmpEchoReply {
    pub address: ipv4::Addr,
    pub status: u32,
    pub rtt: u32,
    pub data_size: u16,
    pub reserved: u16,
    pub data: *const u8,
    pub options: IpOptionInformation,
}