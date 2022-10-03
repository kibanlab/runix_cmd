use std::{env, process::exit};
mod pwd;

fn main() {
    let args: Vec<String> = env::args().collect();

    // オプションが指定されていない場合はヘルプを出力する
    if args.len() == 1 {
        print_help_text();
        exit(0);
    }

    let command = args[1].as_str();

    match command {
        "-v" => version(),
        "--version" => version(),
        "pwd" => pwd::pwd(),
        _ => error_handler(),
    };
}

fn error_handler() {
    println!("error");
}

fn print_help_text() {
    println!("the argument is not given");
}

fn version() {
    println!("runix version is 0.0.3");
}
