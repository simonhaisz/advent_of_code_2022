use crate::network::Network;

pub enum Action<'a> {
    Move(&'a str),
    Open,
}

const TOTAL_TIME: u32 = 30;

pub fn traverse_network(network: &Network, actions: &[Action]) -> u32 {
    let mut time_remaining = TOTAL_TIME;
    let mut total_pressure_released = 0;
    let mut current = network.find("AA").unwrap();

    for action in actions {
        match action {
            Action::Move(next) => {
                time_remaining -= 1;
                let valid = current.validate_move(next);
                if valid {
                    current = network.find(next).unwrap();
                } else {
                    panic!("Invalid move: from '{}' to '{}'", current.name(), next);
                }
            },
            Action::Open => {
                time_remaining -= 1;
                let pressure_released = time_remaining * current.flow_rate() as u32;
                total_pressure_released += pressure_released;
            }
        }
    }

    total_pressure_released
}

pub fn navigate_network(network: &Network) -> Vec<Action> {
    let mut actions = vec![];

    let mut time_remaining = TOTAL_TIME;
    let mut current = network.find("AA").unwrap();
    let mut visited_tunnels = vec![current.name()];
    let mut opened_valves = vec![];

    loop {
        
    }



    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverse() {
        let input = r"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
        ";

        let network = Network::from(input);

        let actions = vec![
            Action::Move("DD"),
            Action::Open,
            Action::Move("CC"),
            Action::Move("BB"),
            Action::Open,
            Action::Move("AA"),
            Action::Move("II"),
            Action::Move("JJ"),
            Action::Open,
            Action::Move("II"),
            Action::Move("AA"),
            Action::Move("DD"),
            Action::Move("EE"),
            Action::Move("FF"),
            Action::Move("GG"),
            Action::Move("HH"),
            Action::Open,
            Action::Move("GG"),
            Action::Move("FF"),
            Action::Move("EE"),
            Action::Open,
            Action::Move("DD"),
            Action::Move("CC"),
            Action::Open,
        ];

        let result = traverse_network(&network, &actions);

        assert_eq!(1651, result);
    }
}