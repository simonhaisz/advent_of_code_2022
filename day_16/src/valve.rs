use lazy_static::lazy_static;
use regex::Regex;

pub struct Valve {
    name: String,
    flow_rate: u8,
    tunnels: Vec<String>,
}

lazy_static! {
    static ref VALVE: Regex = Regex::new(r"^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<tunnels>[A-Z]{2}(?:, [A-Z]{2})*)$").unwrap();
}

impl Valve {
    pub fn new(name: String, flow_rate: u8, tunnels: Vec<String>) -> Self {
        Self { name, flow_rate, tunnels }
    }

    pub fn from(input: &str) -> Self {
        let capture = VALVE.captures(input).expect(&format!("invalid input '{}'", input));

        let name = capture["name"].to_string();
        let flow_rate = capture["flow_rate"].parse::<u8>().unwrap();
        let tunnels = capture["tunnels"].split(", ")
            .map(|s| s.to_string())
            .collect();

        Valve::new(name, flow_rate, tunnels)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn flow_rate(&self) -> u8 {
        self.flow_rate
    }

    pub fn tunnels(&self) -> &[String] {
        &self.tunnels
    }

    pub fn open(&mut self, time_remaining: u8) -> u32 {
        let total_pressure = (self.flow_rate as u32) * (time_remaining as u32);

        total_pressure
    }

    pub fn validate_move(&self, target: &str) -> bool {
        let tunnel = self.tunnels.iter().find(|t| target == *t);
        tunnel.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let valve = Valve::new("A".to_string(), 1, vec![]);

        assert_eq!(1, valve.flow_rate());
    }

    #[test]
    fn from() {
        let valve = Valve::from("Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE");

        assert_eq!("DD", valve.name);
        assert_eq!(20, valve.flow_rate);
        assert_eq!(vec!["CC", "AA", "EE"], valve.tunnels);
    }
}