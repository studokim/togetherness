use clap::Parser;
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

mod db;
mod log;
mod model;
mod rest;
mod timer;

#[derive(Parser, Debug)]
#[command(version, about = "Приложение к празднику Единения 2024", long_about = None)]
struct Args {
    #[arg(env, long, default_value_t = IpAddr::from_str("127.0.0.1")
    .expect("Invalid default IP set. Need to recompile the program."))]
    api_ip: IpAddr,

    #[arg(env, short = 'p', long, value_parser = clap::value_parser!(u16).range(1024..65535))]
    api_port: u16,

    #[arg(env, short, long)]
    database_url: Option<String>,
}

#[tokio::main]
async fn main() {
    log::configure();

    let args = Args::parse();
    if args.database_url == None {
        log::warn!("DATABASE_URL is not set, limited functionality")
    }
    let addr = SocketAddr::from((args.api_ip, args.api_port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Cannot bind the requested address");
    log::info!("Listening on {} ...", addr);

    let router = rest::router::new();
    axum::serve(listener, router)
        .await
        .expect("Cannot start the axum server");
}
