use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "packet.pest"]
pub struct PacketParser;

pub type List = Vec<PacketData>;

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

    pub fn data(&self) -> &List {
        &self.data
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        PacketData::write_list(&mut output, &self.data);

        output
    }
}

pub enum PacketData {
    Integer(u8),
    List(List),
}

impl PacketData {
    pub fn is_integer(&self) -> bool {
        match self {
            PacketData::Integer(_) => true,
            PacketData::List(_) => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            PacketData::Integer(_) => false,
            PacketData::List(_) => true,
        }
    }

    fn write(&self, output: &mut String) {
        match self {
            PacketData::Integer(integer) => PacketData::write_integer(output, integer),
            PacketData::List(list) => PacketData::write_list(output, list),
        }
    }

    fn write_integer(output: &mut String, integer: &u8) {
        output.push_str(&format!("{}", integer))
    }

    fn write_list(output: &mut String, list: &[PacketData]) {
        output.push('[');
        for (i, data) in list.iter().enumerate() {
            if i > 0 {
                output.push(',');
            }
            data.write(output);
        }
        output.push(']');
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