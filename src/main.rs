use log::info;
use crate::app::{init, start_serve};
use crate::cli::run_command;
use crate::conf::get_config;

mod conf;
mod cli;
mod app;
mod errors;
mod types;
mod forms;
mod response;
mod models;
mod utilities;
mod example;
mod redis;
mod services;

#[warn(dead_code)]
fn simple_logger_level(){
    simple_logger::init_with_level(log::Level::Info).unwrap();
}


#[tokio::main]
async fn main() {
    simple_logger_level();
    //let matches = get_matches();
    let config = get_config();
    let config2 = config.clone();
    if let Some(cmd) = config.command {
        match cmd{ 
            conf::Commands::Serve => {
                init(&config2).await;
                start_serve(&config2).await;
            },
            conf::Commands::Cli => {
                println!("cli command");
                run_command("cli", &config2).await;
            }
        }
        info!("Command not match any options");
        return;
    }
}