pub use clap::Parser;
use std::{net::IpAddr, path::PathBuf, str::FromStr};

#[derive(Parser, Debug)]
#[command(version, about = "Приложение к празднику Единения 2024")]
pub struct Args {
    #[arg(env="TOGETHERNESS_IP", short, long, default_value_t = IpAddr::from_str("127.0.0.1")
    .expect("Invalid default IP set. Need to recompile the program."))]
    pub ip: IpAddr,

    #[arg(env="TOGETHERNESS_PORT", short, long, value_parser = clap::value_parser!(u16).range(1024..65535))]
    pub port: u16,

    #[arg(env, short, long)]
    pub database_url: Option<String>,

    #[arg(env, short, long, help = "directory with the compiled React frontend")]
    pub front_dir: PathBuf,

    #[arg(env, short, long, help = "URL, used as the base for redirects")]
    pub root_url: Option<String>,

    #[arg(env, short, long)]
    pub admin_password: String,
}
