use std::collections::HashMap;
use crate::Variable;
use std::process::exit;
use colored::Colorize;

pub fn inc(variables: &mut HashMap<String, Variable>, line: &str) {
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