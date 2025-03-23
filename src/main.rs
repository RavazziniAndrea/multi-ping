extern crate getopts;
use futures::future::join_all;
use getopts::Options;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use std::num::Wrapping;
use std::ops::Range;
use std::os::linux::net;
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
    network: Ipv4Addr,
    range: u8,
    netmask: Ipv4Addr,
    wildcard: Ipv4Addr,
    hosts: u32,
}

fn print_help() {
    println!("Sarebbe bello dai");
}

fn get_subnet_values(subnet: String) -> Result<Network, Box<dyn Error>> {
    let slash_idx = match subnet.find("/") {
        Some(n) => n,
        None => return Err("missing /".into()),
    };

    let mut input_addr = subnet;
    let range = input_addr
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

    let mut network = match Ipv4Addr::from_str(&input_addr) {
        Ok(net) => net,
        Err(_) => {
            return Err("Cannot parse ip address".into());
        }
    };

    let netmask = get_netmask(range);
    let hosts = get_hosts(range);

    network = network & netmask;

    Ok(Network {
        network,
        range,
        netmask,
        wildcard: !netmask,
        hosts,
    })
}

fn get_hosts(range: u8) -> u32 {
    1u32 << (32 - range)
}

fn get_netmask(range: u8) -> Ipv4Addr {
    let mut netmask: u32 = 0xffffffff;
    netmask <<= 32 - range;

    Ipv4Addr::from_bits(netmask)
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

    println!("{:?}", net_data);
    let hosts = get_hosts(net_data.range);
    println!("{hosts}");

    get_netmask(net_data.range);
    let mut tasks = Vec::new();
    for i in 0..hosts {
        let t = tokio::spawn(async move {
            let ip_str = net_data.network.to_string().clone();
            let new_ip = Ipv4Addr::from_str(ip_str.as_str()).unwrap();
            let bits = new_ip.octets();

            //match ping::ping(
            //    IpAddr::from_str("127.0.0.1").unwrap(), //TEST
            //    Some(Duration::from_secs(2)),
            //    None,
            //    None,
            //    None,
            //    None,
            //) {
            //    Ok(r) => r,
            //    Err(e) => panic!("Error: {e}"),
            //};
            println!("OK");
        });
        tasks.push(t);
    }
    join_all(tasks).await;
}
