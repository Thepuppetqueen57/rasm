use std::collections::HashMap;
use crate::Variable;
use std::io;

pub fn get(variables: &mut HashMap<String, Variable>, line: &str) {
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
}