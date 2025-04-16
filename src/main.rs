use crate::app::{init, start_serve};
use crate::cli::run_command;
use crate::conf::{config_from_matches, get_matches};

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

#[warn(dead_code)]
fn simple_logger_level(){
    simple_logger::init_with_level(log::Level::Info).unwrap();
}


#[tokio::main]
async fn main() {
    simple_logger_level();
    let matches = get_matches();
    let config = config_from_matches(&matches);
    if let Some(cmd) = matches.get_one::<String>("cmd") {
        run_command(cmd.as_str(), &config).await;
    }else{
        if matches.is_present("serve") {
            init(&config).await;
            start_serve(&config).await;
        }else{
            println!("Present --serve to start server, or --cmd to run command");
        }
    }
}