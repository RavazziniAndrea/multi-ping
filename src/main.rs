extern crate getopts;
use getopts::Options;
use std::error::Error;
use std::{env, panic};

fn print_help() {
    println!("Sarebbe bello dai");
}

fn get_subnet_values(subnet: String) -> Result<(String, String), Box<dyn Error>> {
    let slash_idx = match subnet.find("/") {
        Some(n) => n,
        None => return Err("missing /")?,
    };

    let mut network = subnet;
    let range = network
        .split_off(slash_idx)
        .trim_start_matches("/")
        .to_string();

    Ok((network, range))
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

    let (network, range) = get_subnet_values(subnet).unwrap();
    println!("{network} -- {range}");
}
