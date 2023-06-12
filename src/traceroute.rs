use crate::error::Error;
use crate::ipresolver::get_ip_addr;
use crate::ping::Pinger;

pub fn trace(addr: String) -> Result<(), Error> {
    // Pinger setup
    let ip = get_ip_addr(addr)?;
    let mut pinger = Pinger::new(ip)?;

    let start = pinger.send(64)?;
    println!("{:?}", pinger.receive(start));

    Ok(())
}