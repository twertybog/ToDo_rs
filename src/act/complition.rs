use std::{io};
pub fn completeness() -> bool{
    println!("Enter task completeness (true/false) default false: ");
    let mut completion = String::new();
    io::stdin().read_line(&mut completion)
        .expect("Error: unable to read user input");
    let completion: bool = match completion.trim().parse(){
        Ok(compl) => {
            compl
        }
        Err(_) => false
    };
    completion
}