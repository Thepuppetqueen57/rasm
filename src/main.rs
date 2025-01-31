use std::process::exit;
use std::fs;
use std::collections::HashMap;

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

    let mut variables: HashMap<String, String> = HashMap::new();

    for line in lines {
        // comarg stands for command argument btw
        let comarg: Vec<&str> = line.split_once(" ")
            .map(|(first, second)| vec![first, second])
            .unwrap_or_else(|| vec![line]);

        if comarg[0] == "out" {
            println!("{}", comarg[1]);
        } else if comarg[0] == "str" {
            let linefunc: Vec<&str> = line.split(" ").collect();

            variables.insert(format!("{}", linefunc[1]), format!("{}", linefunc[2]));
        } else {
            println!("Unknown function");
            exit(1);
        }
    }
}