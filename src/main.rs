extern crate getopts;
use getopts::Options;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;
use std::{env, panic};

/**
 * TODO
 * - Get all possible addresses by range
 * - Ping them all
 * - Show output
 *
 */

#[derive(Debug)]
struct Network {
    net: IpAddr,
    range: u8,
}

fn print_help() {
    println!("Sarebbe bello dai");
}

fn get_subnet_values(subnet: String) -> Result<Network, Box<dyn Error>> {
    let slash_idx = match subnet.find("/") {
        Some(n) => n,
        None => return Err("missing /".into()),
    };

    let mut network = subnet;
    let range = network
        .split_off(slash_idx)
        .trim_start_matches("/")
        .to_string();

    let range = match range.parse::<u8>() {
        Ok(r) => {
            if r > 32 || r < 8 {
                return Err("Range not in boundaries".into());
            }
            r
        }
        Err(_) => return Err("Cannot parse range".into()),
    };

    let net_addr = match Ipv4Addr::from_str(&network) {
        Ok(net) => IpAddr::from(net),
        Err(_) => {
            return Err("Cannot parse ip address".into());
        }
    };

    Ok(Network {
        net: net_addr,
        range: range,
    })
}

fn get_hosts(range: u8) -> u32 {
    1u32 << (32 - range)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print help");
    opts.reqopt("a", "address", "address range", "");

    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_help();
            return;
        }
    };

    if opt_matches.opt_present("h") {
        print_help();
        return;
    }

    let subnet = match opt_matches.opt_str("a") {
        Some(range) => range,
        None => panic!("Range panic"),
    };

    let net_data = match get_subnet_values(subnet) {
        Ok(values) => values,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };

    println!("{:?} -- {:?}", net_data.net, net_data.range);
    let hosts = get_hosts(net_data.range);
    println!("{hosts}");

    for i in [0..hosts] {
        tokio::spawn(async move {
            let x = match ping::ping(
                IpAddr::from_str("127.0.0.1").unwrap(), //TEST
                Some(Duration::from_secs(2)),
                None,
                None,
                None,
                None,
            ) {
                Ok(r) => r,
                Err(e) => panic!("Error: {e}"),
            };
            println!("OK");
        });
    }
}
