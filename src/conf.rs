use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Configuration {
    #[arg(short, long)]
    pub db_uri: String,
    #[arg(short, long)]
    pub port: u16,
    #[arg(short, long)]
    pub redis_uri: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Serve,
    Cli
}

pub fn get_config() -> Configuration {
    Configuration::parse()
}

// pub fn config_from_matches(matches: &ArgMatches) -> Configuration {
//     let db_uri = matches.get_one::<String>("db_uri").expect("need db_uri");
//     let port = matches.get_one::<String>("port").expect("need port");
//     let redis_uri = matches.get_one::<String>("redis").expect("need redis_uri");
//     Configuration{
//         db_uri: db_uri.to_string(),
//         port: port.parse::<u16>().expect("port must be a number"),
//         redis_uri: redis_uri.to_string(),
//     }
// }

