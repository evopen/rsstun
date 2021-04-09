use std::net::IpAddr;

use clap::{App, Arg};

#[derive(Debug)]
struct Arguments {
    stun_host: String,
    stun_port: u16,
    source_ip: IpAddr,
    source_port: u16,
}

fn parse_arguments() -> Arguments {
    let matches = App::new("asdf")
        .arg(
            Arg::with_name("stun-host")
                .long("stun-host")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stun-port")
                .long("stun-port")
                .default_value("3478")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("source-ip")
                .long("source-ip")
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("source-port")
                .long("source-port")
                .default_value("54320"),
        )
        .get_matches();

    let stun_host = matches.value_of("stun-host").unwrap().to_owned();
    let stun_port = matches.value_of("stun-port").unwrap().parse().unwrap();
    let source_ip = matches.value_of("source-ip").unwrap().parse().unwrap();
    let source_port = matches.value_of("source-port").unwrap().parse().unwrap();

    Arguments {
        stun_host,
        stun_port,
        source_ip,
        source_port,
    }
}

fn main() {
    let args = parse_arguments();
    dbg!(args);
}
