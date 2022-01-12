use std::fmt;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CycleLengths {
    pub lengths: Vec<usize>,
    _private: ()
}

impl CycleLengths {
    pub fn new(mut lengths: Vec<usize>) -> CycleLengths {
        lengths = lengths.into_iter().filter(|&l| l >= 2).collect();
        lengths.sort_by_key(|k| usize::MAX - k);
        CycleLengths {
            lengths,
            _private: ()
        }
    }

    pub fn from(s: &str) -> Result<CycleLengths, String> {
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
                    return Err(format!("Failed to parse cycle length {}", piece));
                }
            }
        }

        Ok(CycleLengths::new(parsed_lengths))
    }
}

impl fmt::Display for CycleLengths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.lengths.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_print_cycle_length_elements_sorted() {
        assert_eq!(CycleLengths::new(vec![]).to_string(), "");
        assert_eq!(CycleLengths::new(vec![0, 0, 1]).to_string(), "");
        assert_eq!(CycleLengths::new(vec![2, 3, 3, 4]).to_string(), "4 3 3 2");
    }

    #[test]
    fn should_sort_cycle_lengths() {
        assert_eq!(CycleLengths::new(vec![4, 5]) < CycleLengths::new(vec![2, 6]), true);
        assert_eq!(CycleLengths::new(vec![4, 6]) == CycleLengths::new(vec![6, 4, 1]), true);
        assert_eq!(CycleLengths::new(vec![2, 2, 2]) > CycleLengths::new(vec![2, 2, 1, 1, 1]), true);
    }

    #[test]
    fn should_parse_cycle_lengths() {
        assert_eq!(CycleLengths::from(""), Ok(CycleLengths::new(vec![])));
        assert_eq!(CycleLengths::from("1 2"), Ok(CycleLengths::new(vec![1, 2])));
        assert_eq!(CycleLengths::from("3"), Ok(CycleLengths::new(vec![3])));
        assert_eq!(CycleLengths::from("1 0 1"), Ok(CycleLengths::new(vec![1, 1, 0])));
    }

    #[test]
    fn should_fail_parse_cycle_lengths() {
        assert_eq!(CycleLengths::from("1 -2").is_err(), true);
        assert_eq!(CycleLengths::from("-1 -2").is_err(), true);
        assert_eq!(CycleLengths::from("e").is_err(), true);
    }
}

