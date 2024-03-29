use std::{io, collections::HashMap, process};
mod complition;
use complition::completeness;
use crate::ToDo;

pub fn actions(command: &str, mut tasks: HashMap<String, bool>) -> Result<HashMap<String, bool>,
    Box<dyn std::error::Error>>{
    match command {
        "add"|"a" => {
            println!("Enter task:");
            let mut task = String::new();
            io::stdin().read_line(&mut task)?;
            let task: String = task.trim().parse()
                .expect("Error: unable to read user input");
            tasks.insert(task, false);
        }
        "remove"|"r" => {
            println!("{:#?}", tasks);
            println!("Enter which task you want to delete");
            let mut task = String::new();
            io::stdin().read_line(&mut task)?;
            let task: String = task.trim().parse()
                .expect("Error: unable to read user input");
            tasks.remove(&*task);
        }
        "update"| "u" => {
            println!("{:#?}", tasks);
            println!("Enter which task you want update");
            let mut task = String::new();
            io::stdin().read_line(&mut task)?;
            let task: String = task.trim().parse()
                .expect("Error: unable to read user input");
            tasks.insert(task, completeness());
        }
        "save" => {
            ToDo::save(ToDo{
                tasks: tasks.clone(),
            })?;
            println!("Saved successfully");
        }
        "show"|"s" => {
            println!("{:#?}", tasks);
        }
        "exit"|"q"|"e" => {
            process::exit(0x0100);
        }
        _ => println!("Unsupported command"),
    }
    Ok(tasks)
}
