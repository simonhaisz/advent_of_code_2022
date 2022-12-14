use pest_derive:: Parser;

#[derive(Parser)]
#[grammar = "terminal.pest"]
pub struct TerminalParser;