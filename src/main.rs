use chrono::{DateTime, Utc};
use core::panic;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const FILE_PATH: &str = "./task.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    task: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
impl Task {
    fn print_details(&self) {
        println!("_---------------_");
        println!("Id: {}", self.id);
        println!("task: {}", self.task);
        println!("description: {}", self.description);
        println!("Created at: {}", self.created_at.date_naive());
        println!("Updated at: {}", self.updated_at.date_naive());
    }
}
fn show_task_by_id(task_id: u32) -> Result<Task, String> {
    let tasks = load_json(FILE_PATH);
    for task in tasks {
        if task.id == task_id {
            return Ok(task);
        }
    }
    Err("Task not found".to_owned())

    // let json_string = match serde_json::to_string_pretty(&old_tasks) {
    //     Ok(data) => data,
    //     Err(err) => panic!("Error occured {}",err)
    // };
    // store_json(FILE_PATH, &json_string);
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
    let mut old_task_list = load_json(FILE_PATH);
    old_task_list.push(Task {
        id: (old_task_list.len() + 1) as u32,
        task: task.to_owned(),
        description: description.to_owned(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });
    let json_string = match serde_json::to_string_pretty(&old_task_list) {
        Ok(result) => result,
        Err(error) => panic!(
            "Something went wron while parsing struct to string {}",
            error
        ),
    };
    store_json(FILE_PATH, &json_string);
}
fn show_all_tasks() {
    let tasks_list = load_json(FILE_PATH);
    for task in tasks_list {
        task.print_details();
    }
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
        "show" => show_all_tasks(),
        "show_id" => {
            let id = args.get(2).expect("You did not provided the task id");
            let task =
                show_task_by_id(id.parse().expect("Error while converting task id")).unwrap();
            task.print_details();
        }
        _ => println!("Invalid argument \nUse add,update,delete,show arguments"),
    }
}
