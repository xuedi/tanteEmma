
extern crate mysql;


struct DataStore {
    name: String,
    data: String,
}


fn main() {

    let pool = mysql::Pool::new("mysql://dev:dev@172.20.0.10:3306/dev_test").unwrap();

    let results: Vec<DataStore> = pool.prep_exec("SELECT name, data FROM data_store; ", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (name, data) = mysql::from_row(row);
                DataStore {
                    name: name,
                    data: data,
                }
            }).collect()
        }).unwrap();


    for result in results {
        println!("result.name: {}, result.data: {}", result.name, result.data);
    }
}
