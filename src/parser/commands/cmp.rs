use std::collections::HashMap;
use crate::Variable;

pub fn cmp(
    variables: &mut HashMap<String, Variable>,
    line: &str,
    line_number: u32,
    line_number_at_cmp: &mut u32,
    cmp_statement_size: &mut u32,
    loops: u32
) {
    let linefunc: Vec<&str> = line.split(" ").collect();

    let valid_int: Result<i16, _> = linefunc[2].parse();

    match valid_int {
        Ok(value) => {
            if linefunc[1] != "LOOP" {
                if variables.get(linefunc[1]).unwrap() != &Variable::Int(value) &&
                variables.get(linefunc[1]).unwrap() != &Variable::Byt(value as i8) {
                    *line_number_at_cmp = line_number;
                    *cmp_statement_size = linefunc[3].parse().unwrap();
                }
            } else {
                if loops != value as u32 {
                    *line_number_at_cmp = line_number;
                    *cmp_statement_size = linefunc[3].parse().unwrap();
                }
            }
        }

        _ => {
            if variables.get(linefunc[1]).unwrap() != &Variable::Str(linefunc[2].to_string()) {
                *line_number_at_cmp = line_number;
                *cmp_statement_size = linefunc[3].parse().unwrap();
            }
        }
    }
}