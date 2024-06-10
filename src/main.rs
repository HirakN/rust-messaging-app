use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::thread::sleep;
use std::time::Duration;

const NAME: &str = "user";

//TODO - not used...yet
const SESSION_TOKEN: i32 = 134455;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Message {
    content: String,
    timestamp: String,
    name: String,
    session_token: i32,
}

impl Message {
    fn new(content: String) -> Message {
        let dt = Utc::now();
        Message {
            content,
            name: NAME.to_string(),
            timestamp: dt.to_string(),
            session_token: SESSION_TOKEN,
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  {}\t{}", self.timestamp, self.name, self.content)
    }
}

fn main() {
    let mut old_messages: Vec<Message> = vec![];

    println!("You can type now: ");
    std::thread::spawn(move || {
        let interval = Duration::from_millis(500);
        loop {
            // update messages
            sleep(interval);
            let messages = get_messages();

            let difference: Vec<&Message> = messages
                .iter()
                .filter(|item| !old_messages.contains(item))
                .collect();

            for m in difference {
                println!("{m}")
            }

            old_messages.clone_from(&messages);
        }
    });

    loop {
        let content = take_input();
        let m = Message::new(content);
        store_message(m);
    }
}

fn take_input() -> String {
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();

    user_input.trim().to_owned()
}

fn store_message(message: Message) {
    let client = reqwest::blocking::Client::new();
    let _res = client
        .post("https://lrndvdbaml.execute-api.eu-west-1.amazonaws.com/Prod")
        .body(format!(
            "{{\"name\":\"{}\",\"content\":\"{}\",\"timestamp\":\"{}\",\"session_token\":{}}}",
            message.name, message.content, message.timestamp, message.session_token
        ))
        .send();

    // println!("{:?}", _res);
}

fn get_messages() -> Vec<Message> {
    let body =
        reqwest::blocking::get("https://lrndvdbaml.execute-api.eu-west-1.amazonaws.com/Prod")
            .unwrap()
            .text()
            .unwrap();
    let datas: Vec<Message> = serde_json::from_str(&body).unwrap();
    datas
}
