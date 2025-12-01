use chrono::{DateTime, Utc};
use core::panic;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    task: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

fn store_json(file_path: &str, data: &str) {
    let path = Path::new(file_path);
    let display = path.display();
    let mut file = match File::options().write(true).open(path) {
        Err(error) => panic!("Error occured while creating file {} :: {}", display, error),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(error) => panic!("Something went wrong while writing file {}", error),
        Ok(_) => println!("Sucessfully written to {}", display),
    }
}
fn load_json(file_path: &str) -> Vec<Task> {
    let path = Path::new(file_path);
    let file = match File::options().read(true).open(path) {
        Err(error) => panic!("Something went wrong while reading json file {} ", error),
        Ok(data) => data,
    };
    let task_list: Vec<Task> = match serde_json::from_reader(file) {
        Ok(data) => data,
        _ => {
            vec![]
        }
    };

    task_list
}
fn add_task(task: &str, description: &str) {
    //should first load the previous tasks and then append to that vec
    let mut task_value = vec![Task {
        id: 1,
        task: task.to_owned(),
        description: description.to_owned(),
        created_at: Utc::now(),
        updated_at:Utc::now()
    }];
    let mut old_task_list = load_json("./task.json");
    old_task_list.append(&mut task_value);
    println!("Old json file {:?} ", old_task_list);
    let json_string = match serde_json::to_string(&old_task_list) {
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
