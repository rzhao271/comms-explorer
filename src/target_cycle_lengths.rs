#[derive(PartialEq, Debug)]
pub struct TargetCycleLengths(Vec<u32>);

impl TargetCycleLengths {
    pub fn from(s: &str) -> Result<TargetCycleLengths, String> {
        let pieces = s.split(" ");
        let mut parsed_lengths: Vec<u32> = Vec::new();
        for piece in pieces {
            if piece.len() == 0 {
                continue;
            }
            let parsed_length = piece.parse::<u32>();
            match parsed_length {
                Ok(length) => {
                    if length == 0 {
                        return Err("Target cycle lengths must be >=1".to_owned());
                    }
                    parsed_lengths.push(length);
                },
                Err(e) => {
                    return Err(format!("Failed to parse target cycle length {}", piece));
                }
            }
        }

        if parsed_lengths.len() == 0 {
            return Err("There needs to be at least one target cycle length".to_owned());
        }

        Ok(TargetCycleLengths(parsed_lengths))
    }
}

#[cfg(test)]
mod tests {
    use crate::target_cycle_lengths::*;

    #[test]
    fn should_parse_target_cycle_lengths() {
        assert_eq!(TargetCycleLengths::from("1 2"), Ok(TargetCycleLengths(vec![1, 2])));
        assert_eq!(TargetCycleLengths::from("2 1"), Ok(TargetCycleLengths(vec![2, 1])));
        assert_eq!(TargetCycleLengths::from("3"), Ok(TargetCycleLengths(vec![3])));
    }
    
    #[test]
    fn should_fail_parse_target_cycle_lengths() {
        assert_eq!(TargetCycleLengths::from("").is_err(), true);
        assert_eq!(TargetCycleLengths::from("1 -2").is_err(), true);
        assert_eq!(TargetCycleLengths::from("1 0 1").is_err(), true);
        assert_eq!(TargetCycleLengths::from("-1 -2").is_err(), true);
        assert_eq!(TargetCycleLengths::from("e").is_err(), true);
    }
}

