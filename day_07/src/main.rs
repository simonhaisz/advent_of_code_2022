mod terminal;
mod filesystem;

use std::fs;

use pest::{Parser, iterators::Pair};
use terminal::{TerminalParser, Rule};

fn main() {
    let unparsed_file = fs::read_to_string("./day_07/sample.txt").unwrap();

    let terminal = TerminalParser::parse(Rule::terminal, &unparsed_file)
        .unwrap()
        .next()
        .unwrap();
    
    for line in terminal.into_inner() {
        match line.as_rule() {
            Rule::line => {
                run_line(line);
            }
            _ => println!("unknown line: {}", line.as_str()),
        }
    }
}

fn run_line(line: Pair<Rule>) {
    for component in line.into_inner() {
        match component.as_rule() {
            Rule::command => {
                run_command(component);
            },
            Rule::output => {
                run_output(component);
            },
            _ => println!("unknown command or output: {}", component.as_str()),
        }
    }
}

fn run_command(command: Pair<Rule>) {
    for component in command.into_inner() {
        match component.as_rule() {
            Rule::change_dir => {
                run_change_dir(component);
            },
            Rule::list => {
                run_list(component);
            },
            _ => println!("unknown cd or ls: {}", component.as_str()),
        }
    }
}

fn run_change_dir(change_dir: Pair<Rule>) {
    let mut path: Option<String> = None;
    for component in change_dir.into_inner() {
        match component.as_rule() {
            Rule::path => {
                path = Some(component.as_str().to_string());
            },
            _ => (),
        }
    }
    println!("running command: cd '{}'", path.unwrap());
}

fn run_list(_list: Pair<Rule>) {
    println!("running command: ls");
}

fn run_output(output: Pair<Rule>) {
    for component in output.into_inner() {
        println!("output: {}", component.as_str());
    }
}