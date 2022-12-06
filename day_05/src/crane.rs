use crate::crates::Crates;

pub struct Move {
    moves: u8,
    from: u8,
    to: u8,
}

impl Move {
    pub fn new(moves: u8, from: u8, to: u8) -> Self {
        Move { moves, from, to }
    }
}

pub struct Crane {}

impl Crane {
    pub fn move_crates(crates: &mut Crates, m: Move) {
        for _ in 0..m.moves {
            crates.move_crate(m.from as usize, m.to as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_crates() {
        let mut crates = Crates::from_stacks(vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ]);
        
        Crane::move_crates(&mut crates, Move::new(1, 2, 1));
        Crane::move_crates(&mut crates, Move::new(3, 1, 3));
        Crane::move_crates(&mut crates, Move::new(2, 2, 1));
        Crane::move_crates(&mut crates, Move::new(1, 1, 2));

        assert_eq!("CMZ", crates.top_crates());
    }
}