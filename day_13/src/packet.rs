use std::cell::RefCell;

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "packet.pest"]
pub struct PacketParser;

type List = Vec<PacketData>;

pub struct Packet {
    data: List,
}

impl Packet {
    pub fn from(line: &str) -> Self {
        let parsed_packet = PacketParser::parse(Rule::packet, line)
            .expect(&format!("filed to parse packet '{}'", line))
            .next()
            .unwrap();

        let data = parse_packet_list(parsed_packet);

        Packet { data }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        output.push('[');
        for (i, d) in self.data.iter().enumerate() {
            if i > 0 {
                output.push(',');
            }
            d.write(&mut output);
        }
        output.push(']');

        output
    }
}

pub enum PacketData {
    Integer(u8),
    List(List),
}

impl PacketData {
    fn write(&self, output: &mut String) {
        match self {
            PacketData::Integer(integer) => output.push_str(&format!("{}", integer)),
            PacketData::List(list) => {
                output.push('[');
                for (i, data) in list.iter().enumerate() {
                    if i > 0 {
                        output.push(',');
                    }
                    data.write(output);
                }
                output.push(']');
            },
        }
    }
}

fn parse_packet_list(parsed_list: Pair<Rule>) -> List {
    assert_eq!(Rule::packet, parsed_list.as_rule());

    let mut list = vec![];

    for parsed_data in parsed_list.into_inner() {
        let data = match parsed_data.as_rule() {
            Rule::integer => {
                PacketData::Integer(parse_packet_integer(parsed_data))
            },
            Rule::packet => {
                PacketData::List(parse_packet_list(parsed_data))
            }
        };
        list.push(data);
    }

    list
}

fn parse_packet_integer(parsed_integer: Pair<Rule>) -> u8 {
    assert_eq!(Rule::integer, parsed_integer.as_rule());

    parsed_integer.as_str().parse::<u8>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_parsing(line: &str) {
        let packet = Packet::from(line);
        assert_eq!(line, &packet.to_string());
    }

    #[test]
    fn parse() {
        validate_parsing("[1,1,3,1,1]");
        validate_parsing("[1,1,5,1,1]");

        validate_parsing("[[1],[2,3,4]]");
        validate_parsing("[[1],4]");

        validate_parsing("[9]");
        validate_parsing("[[8,7,6]]");

        validate_parsing("[[4,4],4,4]");
        validate_parsing("[[4,4],4,4,4]");

        validate_parsing("[7,7,7,7]");
        validate_parsing("[7,7,7]");

        validate_parsing("[]");
        validate_parsing("[3]");

        validate_parsing("[[[]]]");
        validate_parsing("[[]]");

        validate_parsing("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        validate_parsing("[1,[2,[3,[4,[5,6,0]]]],8,9]");

    }
}