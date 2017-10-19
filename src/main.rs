
//extern crate mysql;

mod gui;
mod mysql;
mod config;


//struct DataStore {
//    name: String,
//    data: String,
//}


fn main()
{

    let config = config::load("config.toml");

    mysql::connect(); // add config here

    println!("got some data: {}", mysql::get_data());


}
