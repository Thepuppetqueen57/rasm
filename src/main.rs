use std::process::exit;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
enum Variable {
    Str(String),
    Int(i16)
}

fn main() {
    let mut cliargs: Vec<String> = vec![];

    for argument in std::env::args().skip(1) {
        cliargs.push(argument);
    }

    if cliargs.is_empty() {
        println!("Error 1: You did not supply a .rasm file!");
        exit(1);
    }

    let code = fs::read_to_string(cliargs[0].clone())
        .expect("Error 2: Failed to load code. Does the file exist?");

    let lines: Vec<&str> = code.split(";\n").collect();

    let mut variables: HashMap<String, Variable> = HashMap::new();

    for line in lines {
        // comarg stands for command argument btw
        let comarg: Vec<&str> = line.split_once(" ")
            .map(|(first, second)| vec![first, second])
            .unwrap_or_else(|| vec![line]);

        if comarg[0] == "out" {
            println!("{}", comarg[1]);
        } else if comarg[0] == "str" {
            let linefunc: Vec<&str> = line.split(" ").collect();

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
                    println!("Error 4: Variable {} doesnt exist!", comarg[1]);
                    exit(1)
                }
            }

        } else {
            println!("Error 3: Unknown function");
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