extern crate chrono;
extern crate clap;
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json;

use clap::{Arg, App}; // , SubCommand};

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
    let matches = App::new("task_khal")
        .version("1.0")
        .about("Your Taskwarrior tasks into your khal calendar!")
        .author("Francesco Piva <franci@piva.online>")
        .arg(Arg::with_name("project")
             .short("p")
             .long("project")
             .value_name("PROJECT")
             .help("Sets the project to extract form Taskwarrior.")
             .takes_value(true))
        .arg(Arg::with_name("due")
             .short("d")
             .long("due")
             .value_name("DUE")
             .help("Extract only values with due date")
             .takes_value(true))
        .get_matches();

    let project = matches.value_of("project").unwrap_or("");
    //println!("Value for project: {}", project);

    let due = matches.value_of("due").unwrap_or("");
    //println!("Value for due: {}", due);

    let tasks = get_tasks(project.to_string(), due.to_string());

    for task in tasks {
        let due_date = task.due.unwrap(); // .unwrap();
        let desc = task.description.unwrap();
        let task_id = task.uuid;

        // println!("{:?} {:?} {:?}", due_date, desc.replace('\n', ""), task_id);

        let (_res, fname) = create_ics(due_date, desc, task_id);
        match _res {
            Ok(_res) => println!("All good. {:?}", fname),
            Err(_res) => panic!("Something went wrong."),
        }
    }
}

fn create_filename(task_id: &String) -> String {
    //
    // Function that creates the ics filename.
    //
    let mut fname: String = task_id.to_owned();
    fname.push_str(".ics");
    return fname;

}


fn create_ics(due_date: String, desc: String, task_id: String) -> (std::io::Result<()>, String) {
    //
    // Function that creates the ics.
    //
    let filename = create_filename(&task_id);

    let mut calendar = ICalendar::new("2.0", "ics-rs");
    let mut todo = ToDo::new(task_id, due_date);

    todo.push(Summary::new(desc));
    calendar.add_todo(todo);
    let _res = calendar.save_file(filename.clone());

    match _res {
        Ok(_res)  => return (Ok(()), filename),
        Err(_res) => panic!("Couldn't create the ics ! exiting now."),
    }


}


fn get_tasks(project: String, due: String) -> Vec<Taskwarrior> {

    let mut def_project = String::from("project:");
    let mut def_due = String::from("due:");

    def_project.push_str(&project);
    def_due.push_str(&due);

    let output = Command::new("task")
        .arg(def_due)
        .arg(def_project)
        .arg("+READY")
        .arg("export")
        .output()
        .expect("failed to execute process");

    let out = String::from_utf8(output.stdout).unwrap().to_owned();
    let json_out: Vec<Taskwarrior> = serde_json::from_str(&out).unwrap();

    return json_out;
}

fn upload_ics_to_khal() {

}
