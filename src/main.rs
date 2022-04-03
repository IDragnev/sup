use sup::{
    icmp,
    ipv4,
};
use std::{
    env,
    process,
    error::Error,
    thread::sleep,
    time::Duration,
};

fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: sup DEST");
        process::exit(1);
    });
    let dest = ipv4::Addr::parse(&arg)?;

    let data = "Lorem ipsum dolor sit amet";
    println!("Pinging {:?} with {} bytes of data\n", dest, data.len());

    for _ in 0..4 {
        let reply = icmp::Request::new(dest).ttl(128).data(data).send();

        match reply {
            Ok(r) => {
                println!("Reply from {:?}: bytes={} time={:?} TTL={}",
                         r.addr,
                         r.data.len(),
                         r.rtt,
                         r.ttl,
                )
            },
            Err(_) => println!("Something went wrong..."),
        }

        sleep(Duration::from_secs(1));
    }

    Ok(())
}