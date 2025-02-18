use std::process::exit;
use std::fs;
use std::collections::HashMap;
use colored::Colorize;

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
enum Variable {
    Str(String),
    Int(i16),
    Byt(i8),
}

fn split_amount(input: &str, delimiter: &str, n: usize) -> Vec<String> {
    input.splitn(n, delimiter)
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let mut cliargs: Vec<String> = vec![];

    for argument in std::env::args().skip(1) {
        cliargs.push(argument);
    }

    if cliargs.is_empty() {
        println!("{}", "Error 1: You did not supply a .rasm file!".red());
        exit(1);
    }

    let code = fs::read_to_string(cliargs[0].clone())
        .expect(&format!("{}", "Error 2: Failed to load code. Does the file exist?".red()));

    let lines: Vec<&str> = code.split(';').map(|l| l.trim()).collect();

    let mut variables: HashMap<String, Variable> = HashMap::new();

    for line in lines {
        // comarg stands for command argument btw
        let comarg: Vec<&str> = line.split_once(" ")
            .map(|(first, second)| vec![first, second])
            .unwrap_or_else(|| vec![line]);

        if comarg[0] == "out" {
            println!("{}", comarg[1]);
        } else if comarg[0] == "str" {
            let linefunc: Vec<String> = split_amount(line, " ", 3);

            variables.insert(format!("{}", linefunc[1]), Variable::Str(format!("{}", linefunc[2])));
        } else if comarg[0] == "int" {
            let linefunc: Vec<&str> = line.split(" ").collect();

            variables.insert(format!("{}", linefunc[1]), Variable::Int(format!("{}", linefunc[2]).parse::<i16>().unwrap()));
        } else if comarg[0] == "outv" {
            let printed_var = variables.get(comarg[1]);

            match printed_var {
                Some(_variable) => {
                    println!("{:?}", printed_var.unwrap());
                }

                None => {
                    println!("{} {} {}", "Error 4: Variable".red(), comarg[1].blue(), "doesnt exist!".red());
                    exit(1)
                }
            }

        } else if comarg[0] == "bit" {
            let linefunc: Vec<&str> = line.split(" ").collect();

            if linefunc[2].parse::<i8>().unwrap() == 0 || linefunc[2].parse::<i8>().unwrap() == 1 {
                variables.insert(format!("{}", linefunc[1]), Variable::Byt(format!("{}", linefunc[2]).parse::<i8>().unwrap()));
            } else {
                println!("{} {} {} {}", "Error 5: Variable".red(), linefunc[1].blue(), "is a bit yet the value is".red(), linefunc[2].blue());
                exit(1)
            }
        }

        else {
            println!("{}", "Error 3: Unknown function".red());
            exit(1);
        }
    }

    if cliargs.len() == 2 {
        if cliargs[1] == "--debug" || cliargs[1] == "-d" {
            println!("{:?}", variables);
        } else if cliargs[1] == "--help" || cliargs[1] == "-h" {
            println!("Heres a list of tips!");
            println!("For now variables cant have spaces in the value.");
            println!("They will never be able to have spaces in the name.");
        }
    }
}