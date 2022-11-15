use std::{env, process};
use helloworld::Person;

fn main() {

    let args : Vec<String> = env::args().collect();
    

    let person = Person::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = helloworld::run(person){
        println!("Application error: {e}");
        process::exit(1);
    }


}



