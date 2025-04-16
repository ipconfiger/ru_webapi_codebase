use clap::{App, Arg, ArgMatches};
pub struct Configuration {
    pub db_uri: String,
    pub port: u16,
    pub redis_uri: String
}

pub fn get_matches() -> ArgMatches {
    App::new("Server Configuration")
        .arg(Arg::with_name("port")
            .short('p')
            .long("port")
            .help("服务器运行端口")
            .required(true)
            .takes_value(true)
            .default_value("9527"))
        .arg(Arg::with_name("redis")
            .short('r')
            .long("redis")
            .help("Redis连接URI")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("db_uri")
            .short('d')
            .long("db_uri")
            .help("数据库连接URI")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("cmd")
            .short('c')
            .long("cmd")
            .help("执行命令的名字")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("serve")
            .long("serve")
            .help("运行服务器")
            .required(false)
        )
        .get_matches()
}

pub fn config_from_matches(matches: &ArgMatches) -> Configuration {
    let db_uri = matches.get_one::<String>("db_uri").expect("need db_uri");
    let port = matches.get_one::<String>("port").expect("need port");
    let redis_uri = matches.get_one::<String>("redis").expect("need redis_uri");
    Configuration{
        db_uri: db_uri.to_string(),
        port: port.parse::<u16>().expect("port must be a number"),
        redis_uri: redis_uri.to_string(),
    }
}

