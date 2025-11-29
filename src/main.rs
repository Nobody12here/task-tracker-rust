use core::panic;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    task: String,
    description: String,
}
fn store_json(file_path: &str, data: &str) {
    let path = Path::new(file_path);
    let display = path.display();
    let mut file = match File::create(path) {
        Err(error) => panic!("Error occured while creating file {} :: {}", display, error),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(error) => panic!("Something went wrong while writing file {}", error),
        Ok(_) => println!("Sucessfully written to {}", display),
    }
}

fn add_task(task: &str, description: &str) {
    let task_value = Task {
        id: 1,
        task: task.to_owned(),
        description: description.to_owned(),
    };
    let json_string = match serde_json::to_string(&task_value) {
        Ok(result) => result,
        Err(error) => panic!(
            "Something went wron while parsing struct to string {}",
            error
        ),
    };
    store_json("./task.json", &json_string);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = args.get(1).expect("Arguments not provided!").as_str();

    match operation {
        "add" => {
            let task = args.get(2).expect("Task not provided");
            let description: &String = args.get(3).expect("description not provided");
            add_task(task, description) // the deref coercsion will auto convert &String to &str
        }
        "update" => println!("Update"),
        "delete" => println!("delete"),
        "show" => println!("show"),
        _ => println!("Invalid argument \nUse add,update,delete,show arguments"),
    }
}
