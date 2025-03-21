extern crate getopts;
use getopts::Options;
use std::error::Error;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::{env, panic};

#[derive(Debug)]
struct Network {
    net: Ipv4Addr,
    range: i32,
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

    let net_addr = match Ipv4Addr::from_str(&network) {
        Ok(net) => net,
        Err(_) => {
            return Err("Cannot parse ip address".into());
        }
    };

    let range = match range.parse::<i32>() {
        Ok(r) => r,
        Err(_) => return Err("Cannot parse range".into()),
    };

    Ok(Network {
        net: net_addr,
        range: range,
    })
}

fn main() {
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
}
