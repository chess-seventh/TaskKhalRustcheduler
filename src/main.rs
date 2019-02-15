// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use std::io::{self, Write};
// use std::io::{BufRead, BufReader};
use std::process::Command;
// use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Annotations {
    entry: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Taskwarrior {
    id: u16,
    description: String,
    due: String,
    priority: String,
    annotations: Option<Vec<Annotations>>,
    project: String,
    tags: Vec<String>,
    uuid: String,
    urgency: f32,
}

fn main() {
    // run2()
    let output = Command::new("task")
        .arg("due")
        .arg("project:homelab")
        .arg("export")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let out = String::from_utf8(output.stdout).unwrap().to_owned();
        let json_out: Vec<Taskwarrior> = serde_json::from_str(&out).unwrap();
        println!("json_out: {:?}", json_out);
    }
}

fn get_tasks() -> Vec<Taskwarrior> {
    // let mut pro

    let output = Command::new("task")
        .arg("due")
        .arg("project:homelab")
        .arg("export")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let out = String::from_utf8(output.stdout).unwrap().to_owned();
        let json_out: Vec<Taskwarrior> = serde_json::from_str(&out).unwrap();
        println!("json_out: {:?}", json_out);
    }
}
