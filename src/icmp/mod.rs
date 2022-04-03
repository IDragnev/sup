mod icmp_sys;

use std::mem::size_of;
use crate::{
    ipv4,
};

pub struct Request {
    dest: ipv4::Addr,
    ttl: u8,
    timeout: u32,
    data: Option<Vec<u8>>,
}

impl Request {
    pub fn new(dest: ipv4::Addr) -> Self {
        Self {
            dest,
            ttl: 128,
            timeout: 4000,
            data: None,
        }
    }

    pub fn ttl(mut self, ttl: u8) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn data<D>(mut self, data: D) -> Self
        where D: Into<Vec<u8>>, 
    {
        self.data = Some(data.into());
        self
    }

    pub fn send(self) -> Result<(), String> {
        let handle = icmp_sys::IcmpCreateFile();

        let data = self.data.unwrap_or_default();

        let reply_size = size_of::<icmp_sys::IcmpEchoReply>();
        let reply_buf_size = reply_size + 8 + data.len();
        let mut reply_buf = vec![0_u8; reply_buf_size];
    
        let options = icmp_sys::IpOptionInformation {
            ttl: self.ttl,
            tos: 0,
            flags: 0,
            options_size: 0,
            options_data: 0,
        };

        let r = icmp_sys::IcmpSendEcho(
            handle,
            self.dest,
            data.as_ptr(),
            data.len() as u16,
            Some(&options),
            reply_buf.as_mut_ptr(),
            reply_buf_size as u32,
            self.timeout,
        );
    
        match r {
            0 => Err("IcmpSendEcho failed!".to_string()),
            _ => Ok(()),
        }
    }
}