use core::panic;
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn open_file(file_path: &str) {
    let path = Path::new(file_path);
    let dispaly = path.display();
    let mut file = match File::open(path) {
        Err(error) => panic!("Error occured while loading file {} :: {}", dispaly, error),
        Ok(file) => file,
    };
    let mut data_buff = String::new();
    match file.read_to_string(&mut data_buff) {
        Err(error) => panic!("Something went wrong while reading file {}", error),
        Ok(length) => println!("{} contains \n{}\n length {}", dispaly, data_buff, length),
    }
}

fn add_task(task: &str) {
    let json_value = json!({
        "name":"test",
        "age":10,
        "phones":[
            "+92 303 5622496",
            "+92 316 5622496"
        ]
    });
    open_file("./task.json");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = args.get(1).expect("Arguments not provided!").as_str();

    match operation {
        "add" => {
            let task = args.get(2).expect("Task not provided");
            add_task(task) // the deref coercsion will auto convert &String to &str
        }
        "update" => println!("Update"),
        "delete" => println!("delete"),
        "show" => println!("show"),
        _ => println!("Invalid argument \nUse add,update,delete,show arguments"),
    }
}
