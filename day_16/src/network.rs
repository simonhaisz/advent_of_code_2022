use crate::valve::Valve;

pub struct Network {
    valves: Vec<Valve>,
    time_remaining: u8,    
}

impl Network {
    pub fn new(valves: Vec<Valve>) -> Self {
        let time_remaining = 30;
        Self { valves, time_remaining }
    }

    pub fn from(input: &str) -> Self {
        let valves = input.split('\n')
            .filter(|l| !l.trim().is_empty())
            .map(|l| Valve::from(l))
            .collect();

        Network::new(valves)
    }
}

