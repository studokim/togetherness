use clap::Parser;
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

mod rest;

#[derive(Parser, Debug)]
#[command(version, about = "Приложение к празднику Единения 2024", long_about = None)]
struct Args {
    #[arg(long, default_value_t = IpAddr::from_str("127.0.0.1")
                                  .expect("Invalid default IP set. Need to recompile the program."))]
    ip: IpAddr,

    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1024..65535))]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = SocketAddr::from((args.ip, args.port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Cannot bind the requested address");
    println!("Listening on {} ...", addr);

    let router = rest::router::new();
    axum::serve(listener, router)
        .await
        .expect("Cannot start the axum server");
}
