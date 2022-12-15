use std::{fs::File, collections::HashMap};
use csv::{self, Error};

type Record = HashMap<String, String>;


fn main() {
    let _reader = read_from_file("./sample.csv");
}

fn read_from_file(path: &str) -> Result<(), Error> {
    let mut map: HashMap<String, String> = HashMap::new();
    let reader = csv::Reader::from_path(path);

    for result in reader.unwrap().deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
