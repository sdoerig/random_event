use std::{env, process};

mod event;

fn main() {
    let args: Vec<String> = env::args().collect();
    let percent_str: &String = &args[1];
    let percent = percent_str.parse::<u32>().unwrap();
    let event_happend = event::event(percent);
    if  event_happend {
        println!("OH NO - it happend...");
        process::exit(1)
    } else {
        println!("Hi hi hi... ");
        process::exit(0);
    }
}
