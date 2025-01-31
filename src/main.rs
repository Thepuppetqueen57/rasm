use std::process::exit;
use std::fs;

fn main() {
    let mut cliargs: Vec<String> = vec![];

    for argument in std::env::args().skip(1) {
        cliargs.push(argument);
    }

    if cliargs.is_empty() {
        println!("You did not supply a .rasm file!");
        exit(1);
    }

    let code = fs::read_to_string(cliargs[0].clone())
        .expect("Failed to load code. Does the file exist?");

    let lines: Vec<&str> = code.split(";\n").collect();

    for line in lines {
        // comarg stands for command argument btw
        let comarg: Vec<&str> = line.split_once(" ")
            .map(|(first, second)| vec![first, second])
            .unwrap_or_else(|| vec![line]);
        
        let command: &str = comarg[0];
        let argument: &str = comarg[1];

        if command == "out" {
            println!("{}", argument);
        } else {
            println!("Unknown function");
            exit(1);
        }
    }
}