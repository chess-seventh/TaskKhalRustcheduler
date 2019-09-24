// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use std::io::{self, Write};
// use std::io::{BufRead, BufReader};
// use std::process::{Command, Stdio};
//use serde_json::{Result, Value};
extern crate chrono;
// use chrono::prelude::*;
use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json;

// use ics::properties::{Comment, Summary};
use ics::properties::Summary;
use ics::{ICalendar, ToDo};

#[derive(Serialize, Deserialize, Debug)]
struct Annotations {
    entry: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Taskwarrior {
    id: u16,
    description: Option<String>,
    due: Option<String>,
    priority: Option<String>,
    annotations: Option<Vec<Annotations>>,
    project: String,
    tags: Option<Vec<String>>,
    uuid: String,
    urgency: f32,
}

fn main() {
    let mjson = get_tasks();
    let mut calendar = ICalendar::new("2.0", "ics-rs");


    for t in mjson {
        let due_date: &'str = t.due.unwrap(); // .unwrap();
        let desc: &'str = t.description.unwrap();
        let tid: &'str = t.uuid;

        let mut todo = ToDo::new(tid, due_date);
        todo.push(Summary::new(desc));
        // todo.push(Comment::new("Buy her the Imagine Dragons tickets."));

        // println!("{:?}", DateTime::due_date.format("%Y%m%dT%H%M%S%Z"));
        // let custom = DateTime::parse_from_str(&due_date, "%Y%m%dT%H%M%SZ");

        println!("{:?} {:?} {:?}", due_date, desc.replace('\n', ""), tid);

        calendar.add_todo(todo);
    }

}

fn get_tasks() -> Vec<Taskwarrior> {

    let output = Command::new("task")
        .arg("project:home")
        .arg("due")
        .arg("+READY")
        .arg("export")
        .output()
        .expect("failed to execute process");

    let out = String::from_utf8(output.stdout).unwrap().to_owned();
    let json_out: Vec<Taskwarrior> = serde_json::from_str(&out).unwrap();

    return json_out;
}

