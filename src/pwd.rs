use std::env;

pub fn pwd() {
    match env::current_dir() {
        Ok(s) => println!("{}", s.display()),
        Err(err) => eprintln!("{}", err),
    }
}
