use std::io;

fn main() {
    println!("Enter some text to submit:");
    let user_text = take_input();
    let response = store_message(user_text);
    println!("{response:?}");
}

fn take_input() -> String {
    let mut user_input = String::new();

    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();
    user_input
}

fn store_message(text: String){
    let resp = reqwest::blocking::get("http://localhost/api").unwrap();
    resp.text().unwrap();
}