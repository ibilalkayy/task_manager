use crate::commands::commands::{Task, TaskManager};
use clap::Parser;
use serde::Serialize;
use std::fs::File;
use std::io::{Write, Read};

#[derive(Serialize, Debug)]
struct Data {
    id: u32,
    name: String,
    description: String,
}

#[derive(Serialize, Debug)]
struct DataTwo {
    id: u32,
    status: String,
}

pub fn manage_tasks() {
    let args = Task::parse();
    match args.task_manager {
        TaskManager::Add(task) => { 
            let data = Data {
                id: task.id,
                name: task.name,
                description: task.description,
            };

            let serialized = serde_json::to_string(&data).unwrap();
            let mut file = File::create(args.file_name.clone()).expect("failed to create a file");
            file.write_all(serialized.to_string().as_bytes()).expect("failed to write at a file");

            println!("The data is now  successfully added to the JSON file");
        },

        TaskManager::List(task) => {
            if task.id == 2 {
                let mut file = File::open(args.file_name.clone()).expect("failed to open file");
            
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("failed to read the file");
                
                println!("File contents: {}", contents);
            } else {
                println!("File content is not found");
            }
        },
        
        TaskManager::Complete(task) => {
            let data_two = DataTwo {
                id: task.id,
                status: task.status,
            };

            let serialized = serde_json::to_string(&data_two).unwrap();
            let mut file = File::create(args.file_name.clone()).expect("failed to create a file");
            file.write_all(serialized.to_string().as_bytes()).expect("failed to write at a file");

            println!("The task status is successfully added to the JSON file");
        },
        
        TaskManager::Remove(task) => {
            match std::fs::remove_file(&args.file_name) {
                Ok(_) => println!("The task file '{}' was successfully deleted.", args.file_name),
                Err(e) => eprintln!("Failed to remove the file '{}': {}", args.file_name, e),
            }
        }        
    }
}