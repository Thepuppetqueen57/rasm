use std::collections::HashMap;
use std::process::exit;

use colored::Colorize;

use crate::Variable;
use crate::split_amount;
use crate::parser::commands;

pub fn parse_lines(lines: Vec<&str>, variables: &mut HashMap<String, Variable>) {
    let mut loops: u32 = 1;
    let mut line_number: u32 = 0;
    let mut line_number_at_cmp: u32 = 0;
    let mut cmp_statement_size: u32 = 0;

    let mut index = 0;
    let mut goto = false;

    'inf: loop {
        if !goto { index = 0; }
        else { goto = false; }
        for (_i, line) in lines.iter().enumerate().skip(index) {
            index += 1;
            // comarg stands for command argument btw
            let comarg: Vec<&str> = line.split_once(" ")
                .map(|(first, second)| vec![first, second])
                .unwrap_or_else(|| vec![line]);

            if line_number > line_number_at_cmp + cmp_statement_size ||
            cmp_statement_size == 0 {
                match comarg[0] {
                    "out" => println!("{}", comarg[1]),

                    "outv" => {
                        commands::outv::outv(variables, comarg, loops);
                    }

                    "str" => {
                        let linefunc: Vec<String> = split_amount(line, " ", 3);

                        if linefunc[2].starts_with("\"")
                        && linefunc[2].ends_with("\"") {
                            let value: String = linefunc[2].replace("\"", "");
                            variables.insert(format!("{}", linefunc[1]), Variable::Str(format!("{}", value)));
                        } else {
                            println!("{}", "Error 7: Invalid string".red());
                            exit(1)
                        }
                    }

                    "int" => {
                        let linefunc: Vec<&str> = line.split(" ").collect();

                        variables.insert(
                            format!("{}", linefunc[1]),
                            Variable::Int(format!("{}", linefunc[2]).parse::<i16>().unwrap())
                        );
                    }

                    "bit" => {
                        let linefunc: Vec<&str> = line.split(" ").collect();

                        if linefunc[2].parse::<i8>().unwrap() == 0 || linefunc[2].parse::<i8>().unwrap() == 1 {
                            variables.insert(format!("{}", linefunc[1]), Variable::Byt(format!("{}", linefunc[2]).parse::<i8>().unwrap()));
                        } else {
                            println!("{} {} {} {}", "Error 5: Variable".red(), linefunc[1].blue(), "is a bit yet the value is".red(), linefunc[2].blue());
                            exit(1)
                        }
                    }

                    "get" => {
                        commands::get::get(variables, line);
                    }

                    "inc" => {
                        commands::inc::inc(variables, line);
                    }

                    "dec" => {
                        commands::dec::dec(variables, line);
                    }

                    "cmp" => {
                        commands::cmp::cmp(
                            variables,
                            line,
                            line_number,
                            &mut line_number_at_cmp,
                            &mut cmp_statement_size,
                            loops
                        );
                    }

                    "goto" => {
                        let num = comarg[1].parse::<usize>().unwrap();
                        if num <= lines.len() && num > 0 {
                            index = num - 1;
                            goto = true;
                            break;
                        }
                    }

                    "rest" => std::thread::sleep(
                        std::time::Duration::from_millis(comarg[1].parse::<u64>().unwrap())
                    ),

                    "cmd" => {
                        let linefunc: Vec<&str> = line.split(" ").collect();
                        let executable = linefunc[1];
                        let args: Vec<&str> = linefunc[2..].iter()
                            .map(|a| a.trim_end_matches(';'))
                            .collect();
                        std::process::Command::new(executable)
                            .args(args)
                            .spawn()
                            .expect("Failed to execute command");
                    }

                    "//" => {},

                    "HALT" => break 'inf,

                    _ => {
                        println!("{}", "Error 3: Unknown function".red());
                        exit(1);
                    }
                }
            }

            line_number += 1;
            goto = false;
        }
        loops += 1;
    }
}
