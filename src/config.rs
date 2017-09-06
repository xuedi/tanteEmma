

struct Connection {
    host: String,
    user: String,
    pass: String,
    port: i32,
}

struct Database {
    name: String,
}

struct Config {
    app_name: String,
    connection: Connection,
    databases: Vec<Database>,
}


pub fn load() -> Config
{

    // get the return value ready
    let mut config = Config;

    // should be loaded from config file (toml?)
    config.app_name = "TanteEmma";
    config.connection.host = "127.0.0.1";

    // return data
    return config;
}