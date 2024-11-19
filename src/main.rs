mod command;
mod task;
use command::Command;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use task::{Status, Tasks};

fn main() {
    let home_dir = env::var("HOME").expect("$HOME is undefined");
    let path = PathBuf::from(home_dir).join(".todo_data.json");
    let mut f = File::open(&path);
    let mut f_is_e = false;

    let command: Command = Command::collect();
    let mut tasks: Tasks;
    match f {
        Ok(ref mut content) => {
            let mut data = String::new();
            content.read_to_string(&mut data).unwrap();
            tasks = Tasks::from(data);
            f_is_e = true;
        }
        Err(_) => tasks = Tasks::new(),
    }

    if command.command == "" {
        show_all(&tasks);
    } else if command.command == "add" {
        add_task(command.args, &mut tasks);
    } else if command.command == "done" {
        done_task(command.args, &mut tasks);
    } else if command.command == "remove" {
        remove_task(command.args, &mut tasks);
    } else {
        println!("{}: command not found", command.command);
        return;
    }

    if f_is_e {
        let mut f_wr = File::create(&path).unwrap();
        f_wr.write_all(tasks.serialize().as_bytes())
            .expect("write failed");
    } else {
        let mut new_f = File::create_new(&path).unwrap();
        new_f.write_all(tasks.serialize().as_bytes()).unwrap();
    }
}

fn show_all(tasks: &Tasks) {
    let tasks_li = &tasks.tasks;
    for i in tasks_li.iter() {
        let status = if i.status == Status::Done {
            "Done"
        } else {
            "Not Completed"
        };
        println!("{}. {} - {}", i.id + 1, i.title, status);
    }
}

fn add_task(args: Vec<String>, tasks: &mut Tasks) {
    for i in args.iter() {
        tasks.add(i);
    }
}

fn done_task(args: Vec<String>, tasks: &mut Tasks) {
    for i in args.iter() {
        tasks.done(i.parse::<usize>().unwrap() - 1);
    }
}

fn remove_task(args: Vec<String>, tasks: &mut Tasks) {
    for i in args.iter() {
        tasks.remove(i.parse::<usize>().unwrap() - 1);
    }
}
