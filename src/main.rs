use chrono::{DateTime, Utc};
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
fn delete_task(task_id: u32) -> Result<(), std::io::Error> {
    let mut tasks = load_json(FILE_PATH)?;
    tasks.retain(|x: &Task| x.id != task_id);
    println!("{:?}", tasks);
    store_json(FILE_PATH, &serde_json::to_string_pretty(&tasks)?)?;
    Ok(())
}
fn show_task_by_id(task_id: u32) -> Result<Task, String> {
    let tasks = load_json(FILE_PATH).unwrap();
    for task in tasks {
        if task.id == task_id {
            return Ok(task);
        }
    }
    Err("No tasks found for this".to_owned())
}
fn store_json(file_path: &str, data: &str) -> Result<(), std::io::Error> {
    let path = Path::new(file_path);
    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    file.write_all(data.as_bytes())?;
    Ok(())
}
fn load_json(file_path: &str) -> Result<Vec<Task>, std::io::Error> {
    let path = Path::new(file_path);
    let file = File::options().read(true).open(path)?;
    let task_list: Vec<Task> = match serde_json::from_reader(file) {
        Ok(data) => data,
        _ => {
            vec![]
        }
    };

    Ok(task_list)
}
fn update_task(
    task_id: u32,
    updated_task: Option<&str>,
    updated_description: Option<&str>,
) -> Result<(), String> {
    let mut task_list = load_json(FILE_PATH).map_err(|e| e.to_string())?;
    if let Some(task) = task_list.iter_mut().find(|x| x.id == task_id) {
        if let Some(updated_description) = updated_description {
            task.description = updated_description.to_owned()
        }
        if let Some(updated_task) = updated_task {
            task.task = updated_task.to_owned()
        }
    } else {
        return Err(format!("No task found with {} task id", task_id));
    }
    store_json(
        FILE_PATH,
        &serde_json::to_string_pretty(&task_list).expect("msg"),
    )
    .map_err(|err| err.to_string())?;
    Ok(())
}
fn add_task(task: &str, description: &str) -> Result<(), std::io::Error> {
    let mut old_task_list = load_json(FILE_PATH)?;
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
    store_json(FILE_PATH, &json_string)?;
    Ok(())
}
fn show_all_tasks() -> Result<(), std::io::Error> {
    let tasks_list = load_json(FILE_PATH)?;
    for task in tasks_list {
        task.print_details();
    }
    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = args.get(1).expect("Arguments not provided!").as_str();

    match operation {
        "add" => {
            let task = args.get(2).expect("Task not provided");
            let description: &String = args.get(3).expect("description not provided");
            add_task(task, description); // the deref coercsion will auto convert &String to &str
        }
        "update" => {
            let id: u32 = args
                .get(2)
                .expect("No task id provided")
                .parse()
                .expect("error while parsing string to int.");
            let task = args.get(3).map(|s| s.as_str());
            let description = args.get(4).map(|s| s.as_str());
            update_task(id, task, description);
        }
        "delete" => {
            let id = args.get(2).expect("You did not provided the task id");
            delete_task(id.parse().unwrap());
        }
        "show" => {
            show_all_tasks();
        }
        "show_id" => {
            let id = args.get(2).expect("You did not provided the task id");
            let task =
                show_task_by_id(id.parse().expect("Error while converting task id")).unwrap();
            task.print_details();
        }
        _ => println!("Invalid argument \nUse add,update,delete,show arguments"),
    }
}
