use lazy_static::lazy_static;
use regex::Regex;

pub struct Valve {
    name: String,
    flow_rate: u8,
    tunnels: Vec<String>,
    open: bool,
}

lazy_static! {
    static ref VALVE: Regex = Regex::new(r"^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<flow_rate>\d+); tunnels lead to valves (?P<tunnels>[A-Z]{2}(?:, [A-Z]{2})*)$").unwrap();
}

impl Valve {
    pub fn new(name: String, flow_rate: u8, tunnels: Vec<String>) -> Self {
        let open = false;
        Self { name, flow_rate, tunnels, open }
    }

    pub fn from(input: &str) -> Self {
        let capture = VALVE.captures(input).unwrap();

        let name = capture["name"].to_string();
        let flow_rate = capture["flow_rate"].parse::<u8>().unwrap();
        let tunnels = capture["tunnels"].split(", ")
            .map(|s| s.to_string())
            .collect();

        Valve::new(name, flow_rate, tunnels)
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn open(&mut self, time_remaining: u8) -> u32 {
        let total_pressure = (self.flow_rate as u32) * (time_remaining as u32);

        self.open = true;

        total_pressure
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let valve = Valve::new("A".to_string(), 1, vec![]);

        assert_eq!(false, valve.open);
    }

    #[test]
    fn from() {
        let valve = Valve::from("Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE");

        assert_eq!("DD", valve.name);
        assert_eq!(20, valve.flow_rate);
        assert_eq!(vec!["CC", "AA", "EE"], valve.tunnels);
    }

    #[test]
    fn open() {
        let mut valve = Valve::from("Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE");

        let pressure_reduction = valve.open(13);

        assert_eq!(true, valve.open);
        assert_eq!(260, pressure_reduction);
    }
}