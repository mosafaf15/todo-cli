use std::usize;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum Status {
    NotComplete,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Task {
    fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            status: Status::NotComplete,
        }
    }
    fn ch_status(&mut self, status: Status) {
        self.status = status;
    }
}

impl Tasks {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }
    pub fn from(data: String) -> Self {
        from_str(&data).unwrap()
    }
    pub fn serialize(&self) -> String {
        to_string(self).unwrap()
    }
    pub fn add(&mut self, title: &str) {
        let task_id = self.tasks.len();
        let task = Task::new(task_id, String::from(title));
        self.tasks.push(task);
    }
    pub fn done(&mut self, id: usize) {
        self.tasks[id].ch_status(Status::Done);
    }
    pub fn remove(&mut self, id: usize) {
        let index = self.tasks.iter().position(|i| i.id == id).unwrap();
        self.tasks.remove(index);
        // for i in id..self.tasks.len(){
        //     self.tasks[i].id -= 1;
        // }
    }
}
