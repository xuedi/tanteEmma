

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
    app_version: String,
    app_name: String,
    //connection: Connection,
    //databases: Vec<Database>,
}

/*
pub fn load() -> Config
{

    // prepare the return struct
    let mut config = Config {app_name: 'dfgdfg'};

    // should be loaded from config file (toml?)
    config.app_name = "TanteEmma";
    config.connection.host = "127.0.0.1";
    config.connection.user = "test";
    config.connection.pass = "test";
    config.connection.port = 3306;

    println!("x is {}", config.app_name);

    // return data
    return config;
}
*/

/// from the net
pub fn load(path: String) -> Config {
    let mut config_toml = String::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_)  => {
            error!("Could not find config file, using default!");
            return Config::new();
        }
    };

    file.read_to_string(&mut config_toml)
            .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let mut parser = Parser::new(&config_toml);
    let toml = parser.parse();

    if toml.is_none() {
        for err in &parser.errors {
            let (loline, locol) = parser.to_linecol(err.lo);
            let (hiline, hicol) = parser.to_linecol(err.hi);
            println!("{}:{}:{}-{}:{} error: {}",
                     path, loline, locol, hiline, hicol, err.desc);
        }
        panic!("Exiting server");
    }

    let config = Value::Table(toml.unwrap());
    match toml::decode(config) {
        Some(t) => t,
        None => panic!("Error while deserializing config")
    }
}