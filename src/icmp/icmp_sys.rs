use crate::{
    ipv4,
    lib_loader::Library,
};
use std::ffi::c_void;

pub type Handle = *const c_void;

#[allow(non_snake_case)]
type IcmpCreateFile = extern "stdcall" fn() -> Handle;

#[allow(non_snake_case)]
pub fn IcmpCreateFile() -> Handle {
    let iphlp = Library::new("IPHLPAPI.dll").unwrap();
    let IcmpCreateFile: IcmpCreateFile = iphlp.get_proc("IcmpCreateFile").unwrap();
    IcmpCreateFile()
    // TODO: release lib resources
}

type IcmpSendEcho = extern "stdcall" fn(
    handle: Handle,
    dest: ipv4::Addr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32;

#[allow(non_snake_case)]
pub fn IcmpSendEcho(
    handle: Handle,
    dest: ipv4::Addr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32 {
    let iphlp = Library::new("IPHLPAPI.dll").unwrap();
    let IcmpSendEcho: IcmpSendEcho = iphlp.get_proc("IcmpSendEcho").unwrap();
    IcmpSendEcho(
        handle,
        dest,
        request_data,
        request_size,
        request_options,
        reply_buffer,
        reply_size,
        timeout,
    )
    // TODO: release lib resources
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