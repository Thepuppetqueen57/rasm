use std::collections::HashMap;
use std::process::exit;
use std::io;

use colored::Colorize;

use crate::Variable;
use crate::split_amount;

pub fn parse_lines(lines: Vec<&str>, variables: &mut HashMap<String, Variable>) {
    let mut loops: u32 = 1;
    let mut line_number: u32 = 0;
    let mut line_number_at_cmp: u32 = 0;
    let mut cmp_statement_size: u32 = 0;

    let mut index = 0;

    'inf: loop {
        for (i, line) in lines.iter().enumerate().skip(index) {
        index += 1;
            // comarg stands for command argument btw
            let comarg: Vec<&str> = line.split_once(" ")
                .map(|(first, second)| vec![first, second])
                .unwrap_or_else(|| vec![line]);

            if line_number > line_number_at_cmp + cmp_statement_size ||
            cmp_statement_size == 0 {
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
                            Some(variable) => {
                                if let Variable::Str(value) = variable {
                                    println!("{}", value);
                                } else if let Variable::Int(value) = variable {
                                    println!("{}", value);
                                } else if let Variable::Byt(value) = variable {
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
                        Some(variable) => {
                            if let Variable::Int(value) = variable {
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

                else if comarg[0] == "dec" {
                    let linefunc: Vec<&str> = line.split(" ").collect();

                    let var = variables.get(linefunc[1]);

                    match var {
                        Some(variable) => {
                            if let Variable::Int(value) = variable {
                                let new_value = value - linefunc[2].parse::<i16>().unwrap();

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
                            if linefunc[1] != "LOOP" {
                                if variables.get(linefunc[1]).unwrap() != &Variable::Int(value) &&
                                variables.get(linefunc[1]).unwrap() != &Variable::Byt(value as i8) {
                                    line_number_at_cmp = line_number;
                                    cmp_statement_size = linefunc[3].parse().unwrap();
                                }
                            } else {
                                if loops != value as u32 {
                                    line_number_at_cmp = line_number;
                                    cmp_statement_size = linefunc[3].parse().unwrap();
                                }
                            }
                        }

                        _ => {
                            if variables.get(linefunc[1]).unwrap() != &Variable::Str(linefunc[2].to_string()) {
                                line_number_at_cmp = line_number;
                                cmp_statement_size = linefunc[3].parse().unwrap();
                            }
                        }
                    }
                }

                else if comarg[0] == "get" {
                    let linefunc: Vec<&str> = line.split(" ").collect();

                    let var = variables.get(linefunc[1]);
                    match var {
                        Some(_variable) => {
                            let mut new_value = String::new();
                            io::stdin().read_line(&mut new_value).expect("Error 6: Failed to read line");
                            new_value = new_value.trim().to_string();
                            variables.remove(linefunc[1]);
                            variables.insert(linefunc[1].to_string(), Variable::Str(new_value));
                        }

                        None => {
                            let mut new_value = String::new();
                            io::stdin().read_line(&mut new_value).expect("Error 6: Failed to read line");
                            new_value = new_value.trim().to_string();
                            variables.insert(linefunc[1].to_string(), Variable::Str(new_value));
                        }
                    }
                } else if comarg[0] == "goto" {
                    let num = comarg[1].parse::<usize>().unwrap();
                    if num < lines.len() && num > 0 {
                        index = num - 1;
                        break;
                    } else {
                        println!("{}", "Error 7: Line doesn't exist!");
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
