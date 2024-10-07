use std::ops::RangeInclusive;

use clap::Parser;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn validate_port(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid port number"))?;

    if PORT_RANGE.contains(&port) {
        return Ok(port as u16);
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

fn validate_ip_addr(s: &str) -> Result<[u8; 4], String> {
    let mut addr: [u8; 4] = [127, 0, 0, 1];
    
    let split_ip = s.split('.').enumerate();

    for (indx, addr_part) in split_ip {
        if indx >= 4 { 
            return Err(
                format!(
                    "Invalid IP address: {s}"
                )
            );
        }
        
        if let Ok(ip_value) = addr_part.parse::<u8>() {
            addr[indx] = ip_value;
        } else { 
            return Err(
                format!("Invalid Ip address: {s}")
            )
        }
    }
    
    return Ok(addr);
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_parser = validate_ip_addr, default_value = "127.0.0.1")]
    pub addr: [u8; 4],

    #[arg(short, long, value_parser = validate_port, default_value_t = 3000)]
    pub port: u16,
}
