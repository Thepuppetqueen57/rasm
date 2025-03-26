use std::process::exit;
use std::fs;
use std::collections::HashMap;
use colored::Colorize;

pub mod parser;
use parser::main_parser::parse_lines;

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Variable {
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

    // let mut loops: u32 = 1;

    parse_lines(lines, &mut variables);

    if cliargs.len() == 2 {
        if cliargs[1] == "--debug" || cliargs[1] == "-d" {
            println!("{:?}", variables);
        } else if cliargs[1] == "--help" || cliargs[1] == "-h" {
            println!("Heres a list of tips!");
            println!("For now variables cant have spaces in the value.");
            println!("They will never be able to have spaces in the name.");
            println!("--help has been deprecated!");
            println!("Its a way better idea to read the docs!");
        }
    }
}