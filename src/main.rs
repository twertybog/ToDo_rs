use std::{io, collections::HashMap, env};
use serde_json;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
mod act;
use act::actions;
struct ToDo{
    tasks: HashMap<String, bool>,
}
impl ToDo {
    fn new() -> Result<ToDo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        match serde_json::from_reader(f) {
            Ok(tasks) => Ok(ToDo { tasks }),
            Err(e) if e.is_eof() => Ok(ToDo {
                tasks: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    pub fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.tasks)?;
        Ok(())
    }
}
fn main(){

    let arguments: String = env::args().collect();
    let mut todo = ToDo::new().expect("Initialisation of db failed");
    if arguments.contains("--cli"){
        loop {
            println!("Enter command (add/remove/update/show/save/exit): ");
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Error: unable to read user input");
            let mut command: String = command.trim().parse()
                .expect("Error: unable to read user input");
            todo.tasks = actions(command.as_mut_str(), todo.tasks).unwrap();
        }
    }
    else{
        let app = Application::builder()
            .application_id("org.ToDo")
            .build();

        app.connect_activate(|app| {
            let win = ApplicationWindow::builder()
                .application(app)
                .default_height(600)
                .default_width(800)
                .title("ToDo")
                .build();
            
                win.show();

        });
        app.run();
    }
}
