use crate::valve::Valve;

pub struct Network {
    valves: Vec<Valve>,
}

impl Network {
    pub fn new(valves: Vec<Valve>) -> Self {
        Self { valves }
    }

    pub fn from(input: &str) -> Self {
        let valves = input.split('\n')
            .filter(|l| !l.trim().is_empty())
            .map(|l| Valve::from(l.trim()))
            .collect();

        Network::new(valves)
    }

    pub fn validate_move(&self, origin: &str, destination: &str) -> bool {
        let valve = self.valves.iter().find(|v| v.name() == origin);

        if valve.is_some() {
            let tunnel = valve.unwrap();
            let tunnel = tunnel.tunnels().iter().find(|t| destination == *t);
            tunnel.is_some()
        } else {
            false
        }
    }

    pub fn find(&self, name: &str) -> Option<&Valve> {
        self.valves.iter().find(|v| v.name() == name)
    }
}

