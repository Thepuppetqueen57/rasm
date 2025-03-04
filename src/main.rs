use std::process::exit;
use std::fs;
use std::collections::HashMap;
use colored::Colorize;
use std::io;

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

fn parse_lines(lines: Vec<&str>, variables: &mut HashMap<String, Variable>) {
    let mut loops: u32 = 1;
    let mut line_number: u32 = 0;
    let mut line_number_at_if: u32 = 0;
    let mut if_statement_size: u32 = 0;

    'inf: loop {
        for line in &lines {
            // comarg stands for command argument btw
            let comarg: Vec<&str> = line.split_once(" ")
                .map(|(first, second)| vec![first, second])
                .unwrap_or_else(|| vec![line]);

            if line_number > line_number_at_if + if_statement_size ||
            if_statement_size == 0 {
                if comarg[0] == "out" {
                    println!("{}", comarg[1]);
                }
    
                else if comarg[0] == "str" {
                    let linefunc: Vec<String> = split_amount(line, " ", 3);
        
                    variables.insert(format!("{}", linefunc[1]), Variable::Str(format!("{}", linefunc[2])));
                }
    
                else if comarg[0] == "int" {
                    let linefunc: Vec<&str> = line.split(" ").collect();
        
                    variables.insert(format!("{}", linefunc[1]), Variable::Int(format!("{}", linefunc[2]).parse::<i16>().unwrap()));
                }
    
                else if comarg[0] == "outv" {
                    if comarg[1] != "LOOP" {
                        let printed_var = variables.get(comarg[1]);
    
                        match printed_var {
                            Some(_variable) => {
                                if let Variable::Str(value) = variables.get(comarg[1]).unwrap() {
                                    println!("{}", value);
                                } else if let Variable::Int(value) = variables.get(comarg[1]).unwrap() {
                                    println!("{}", value);
                                } else if let Variable::Byt(value) = variables.get(comarg[1]).unwrap() {
                                    println!("{}", value);
                                }
                            }
    
                            None => {
                                println!("{} {} {}", "Error 4: Variable".red(), comarg[1].blue(), "doesnt exist!".red());
                                exit(1)
                            }
                        }
                    } else {
                        println!("{}", loops);
                    }
                }
    
                else if comarg[0] == "bit" {
                    let linefunc: Vec<&str> = line.split(" ").collect();
        
                    if linefunc[2].parse::<i8>().unwrap() == 0 || linefunc[2].parse::<i8>().unwrap() == 1 {
                        variables.insert(format!("{}", linefunc[1]), Variable::Byt(format!("{}", linefunc[2]).parse::<i8>().unwrap()));
                    } else {
                        println!("{} {} {} {}", "Error 5: Variable".red(), linefunc[1].blue(), "is a bit yet the value is".red(), linefunc[2].blue());
                        exit(1)
                    }
                }
    
                else if comarg[0] == "HALT" {
                    break 'inf;
                }
    
                else if comarg[0] == "//" {}
    
                else if comarg[0] == "inc" {
                    let linefunc: Vec<&str> = line.split(" ").collect();
    
                    let var = variables.get(linefunc[1]);
    
                    match var {
                        Some(_variable) => {
                            if let Variable::Int(value) = variables.get(linefunc[1]).unwrap() {
                                let new_value = value + linefunc[2].parse::<i16>().unwrap();
    
                                variables.remove(linefunc[1]);
                                variables.insert(linefunc[1].to_string(), Variable::Int(new_value));
                            }
                        }
    
                        None => {
                            println!("{} {} {}", "Error 4: Variable".red(), linefunc[1].blue(), "doesnt exist!".red());
                            exit(1)
                        }
                    }
                }
    
                else if comarg[0] == "rest" {
                    std::thread::sleep(std::time::Duration::from_millis(comarg[1].parse::<u64>().unwrap()));
                }
    
                else if comarg[0] == "cmp" {
                    let linefunc: Vec<&str> = line.split(" ").collect();

                    let valid_int: Result<i16, _> = linefunc[2].parse();

                    match valid_int {
                        Ok(value) => {
                            if variables.get(linefunc[1]).unwrap() != &Variable::Int(value) &&
                            variables.get(linefunc[1]).unwrap() != &Variable::Byt(value as i8) {
                                line_number_at_if = line_number;
                                if_statement_size = linefunc[3].parse().unwrap();
                            }
                        }

                        _ => {
                            if variables.get(linefunc[1]).unwrap() != &Variable::Str(linefunc[2].to_string()) {
                                line_number_at_if = line_number;
                                if_statement_size = linefunc[3].parse().unwrap();
                            }
                        }
                    }
                }

                else if comarg[0] == "get" {
                    let linefunc: Vec<&str> = line.split(" ").collect();

                    let var = variables.get(linefunc[1]);
                    match var {
                        Some(_variable) => {
                            if let Variable::Str(value) = variables.get(linefunc[1]).unwrap() {
                                let mut new_value = String::new();
                                io::stdin().read_line(&mut new_value).expect("Error 6: Failed to read line");
                                new_value = new_value.trim().to_string();
                                variables.remove(linefunc[1]);
                                variables.insert(linefunc[1].to_string(), Variable::Str(new_value));
                            }
                        }

                        None => {
                            let mut new_value = String::new();
                            io::stdin().read_line(&mut new_value).expect("Error 6: Failed to read line");
                            new_value = new_value.trim().to_string();
                            variables.insert(linefunc[1].to_string(), Variable::Str(new_value));
                        }
                    }
                }

                else {
                    println!("{}", "Error 3: Unknown function".red());
                    exit(1);
                }
            }

            line_number += 1
        }

        loops += 1
    }
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

    // let mut loops: u32 = 1;

    parse_lines(lines, &mut variables);

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