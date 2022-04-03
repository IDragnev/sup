use sup::{
    icmp,
    ipv4,
};
use std::{
    env,
    process,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: sup DEST");
        process::exit(1);
    });
    let dest = ipv4::Addr::parse(&arg)?;

    icmp::Request::new(dest)
        .ttl(128)
        .data("Lorem ipsum dolor sit amet")
        .send()?;

    Ok(())
}