use crate::packet::{List, Packet, PacketData::{Integer as IntData, List as ListData}};

pub fn validate_packets(a: &Packet, b: &Packet) -> bool {
    validate_list(a.data(), b.data()).unwrap()
}

fn validate_integer(a: &u8, b: &u8) -> Option<bool> {
    if a < b {
        Some(true)
    } else if a == b {
        None
    } else {
        Some(false)
    }
}

fn validate_list(a: &List, b: &List) -> Option<bool> {
    let mut a_it = a.iter();
    let mut b_it = b.iter();

    loop {
        let a_current = a_it.next();
        let b_current = b_it.next();

        if a_current.is_none() && b_current.is_none() {
            break None;
        } else if a_current.is_none() {
            break Some(true);
        } else if b_current.is_none() {
            break Some(false);
        } else {
            let a_current = a_current.unwrap();
            let b_current = b_current.unwrap();

            let current_valid = if let (IntData(a_integer), IntData(b_integer)) = (a_current, b_current) {
                validate_integer(a_integer, b_integer)
            } else if let (ListData(a_list), ListData(b_list)) = (a_current, b_current) {
                validate_list(a_list, b_list)
            } else {
                match a_current {
                    IntData(a_integer) => {
                        if let ListData(b_list) = b_current {
                            let a_list = vec![IntData(*a_integer)];
                            validate_list(&a_list, b_list)
                        } else {
                            panic!("impossible")
                        }
                    },
                    ListData(a_list) => {
                        if let IntData(b_integer) = b_current {
                            let b_list = vec![IntData(*b_integer)];
                            validate_list(a_list, &b_list)
                        } else {
                            panic!("impossible")
                        }
                    }
                }
            };

            if current_valid.is_some() {
                break current_valid;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let a = Packet::from("[1,1,3,1,1]");
        let b = Packet::from("[1,1,5,1,1]");
        assert_eq!(true, validate_packets(&a, &b));

        let a = Packet::from("[[1],[2,3,4]]");
        let b = Packet::from("[[1],4]");
        assert_eq!(true, validate_packets(&a, &b));

        let a = Packet::from("[9]");
        let b = Packet::from("[[8,7,6]]");
        assert_eq!(false, validate_packets(&a, &b));

        let a = Packet::from("[[4,4],4,4]");
        let b = Packet::from("[[4,4],4,4,4]");
        assert_eq!(true, validate_packets(&a, &b));

        let a = Packet::from("[7,7,7,7]");
        let b = Packet::from("[7,7,7]");
        assert_eq!(false, validate_packets(&a, &b));

        let a = Packet::from("[]");
        let b = Packet::from("[3]");
        assert_eq!(true, validate_packets(&a, &b));

        let a = Packet::from("[[[]]]");
        let b = Packet::from("[[]]");
        assert_eq!(false, validate_packets(&a, &b));

        let a = Packet::from("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let b = Packet::from("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(false, validate_packets(&a, &b));
    }
}