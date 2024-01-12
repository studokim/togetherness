use clap::Parser;
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

mod db;
mod log;
mod model;
mod rest;
mod router;
mod static_server;
mod timer;

#[derive(Parser, Debug)]
#[command(version, about = "Приложение к празднику Единения 2024")]
pub struct Args {
    #[arg(env="TOGETHERNESS_IP", short, long, default_value_t = IpAddr::from_str("127.0.0.1")
    .expect("Invalid default IP set. Need to recompile the program."))]
    ip: IpAddr,

    #[arg(env="TOGETHERNESS_PORT", short, long, value_parser = clap::value_parser!(u16).range(1024..65535))]
    port: u16,

    #[arg(env, short, long)]
    database_url: Option<String>,

    #[arg(env, short, long, help = "directory with the compiled React frontend")]
    front_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    log::configure();

    let args = Args::parse();
    if args.database_url == None {
        log::warn!("DATABASE_URL is not set, limited functionality")
    }
    let addr = SocketAddr::from((args.ip, args.port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Cannot bind the requested address");
    log::info!("Listening on {} ...", addr);

    let router = router::new();
    axum::serve(listener, router)
        .await
        .expect("Cannot start the axum server");
}
