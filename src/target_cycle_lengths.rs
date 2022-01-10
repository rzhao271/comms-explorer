#[derive(PartialEq, Debug)]
pub struct TargetCycleLengths {
    pub target_lengths: Vec<usize>
}

impl TargetCycleLengths {
    pub fn new(mut target_lengths: Vec<usize>) -> TargetCycleLengths {
        if target_lengths.len() == 0 {
            return TargetCycleLengths {
                target_lengths: vec![0]
            };
        }
        target_lengths.sort();
        TargetCycleLengths {
            target_lengths
        }
    }

    pub fn from(s: &str) -> Result<TargetCycleLengths, String> {
        let pieces = s.split(" ");
        let mut parsed_lengths: Vec<usize> = Vec::new();
        for piece in pieces {
            if piece.len() == 0 {
                continue;
            }
            let parsed_length = piece.parse::<usize>();
            match parsed_length {
                Ok(length) => {
                    parsed_lengths.push(length);
                },
                Err(_) => {
                    return Err(format!("Failed to parse target cycle length {}", piece));
                }
            }
        }

        Ok(TargetCycleLengths::new(parsed_lengths))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_target_cycle_lengths() {
        assert_eq!(TargetCycleLengths::from("2 1"), Ok(TargetCycleLengths { target_lengths: vec![1, 2] }));
        assert_eq!(TargetCycleLengths::from("2 1"), Ok(TargetCycleLengths::new(vec![1, 2])));
    }

    #[test]
    fn should_parse_target_cycle_lengths() {
        assert_eq!(TargetCycleLengths::from(""), Ok(TargetCycleLengths::new(vec![])));
        assert_eq!(TargetCycleLengths::from("1 2"), Ok(TargetCycleLengths::new(vec![1, 2])));
        assert_eq!(TargetCycleLengths::from("3"), Ok(TargetCycleLengths::new(vec![3])));
        assert_eq!(TargetCycleLengths::from("1 0 1"), Ok(TargetCycleLengths::new(vec![0, 1, 1])));
    }

    #[test]
    fn should_fail_parse_target_cycle_lengths() {
        assert_eq!(TargetCycleLengths::from("1 -2").is_err(), true);
        assert_eq!(TargetCycleLengths::from("-1 -2").is_err(), true);
        assert_eq!(TargetCycleLengths::from("e").is_err(), true);
    }
}

