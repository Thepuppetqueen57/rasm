use std::collections::HashMap;
use crate::Variable;
use std::process::exit;
use colored::Colorize;

pub fn outv(variables: &HashMap<String, Variable>, comarg: Vec<&str>, loops: u32) {
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