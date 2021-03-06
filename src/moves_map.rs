use std::collections::BTreeMap;

use crate::permutation::Permutation;

#[derive(PartialEq, Debug)]
pub struct MovesMap {
    pub record: BTreeMap<String, Permutation>,
    _private: ()
}

impl MovesMap {
    pub fn new(record: BTreeMap<String, Permutation>) -> MovesMap {
        MovesMap {
            record,
            _private: ()
        }
    }

    pub fn from(s: &str) -> Result<MovesMap, String> {
        let mut map: BTreeMap<String, Permutation> = BTreeMap::new();
        for line in s.lines() {
            if line.trim().len() == 0 || line.starts_with("//") {
                continue;
            }
            let mut pieces = line.split(" ");
            let move_id = pieces.next().unwrap();
            let cycles = pieces.collect::<Vec<&str>>().join(" ");
            let permutation = match Permutation::from(&cycles) {
                Ok(p) => p,
                Err(e) => return Err(e)
            };
            map.insert(move_id.to_owned(), permutation);
        }
        Ok(MovesMap::new(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_moves_map() {
        assert_eq!(MovesMap::from(""), Ok(MovesMap::new(BTreeMap::new())));
        assert_eq!(MovesMap::from("M"), Ok(MovesMap::new(BTreeMap::from([("M".to_owned(), Permutation::new(vec![]))]))));
        assert_eq!(MovesMap::from("R (1)"),
            Ok(MovesMap::new(BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1]]))]))));
        assert_eq!(MovesMap::from("R (1 2)"),
            Ok(MovesMap::new(BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2]]))]))));
        assert_eq!(MovesMap::from("R (1 2) (3 4)"),
            Ok(MovesMap::new(BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2], vec![3, 4]]))]))));
        assert_eq!(MovesMap::from("R (1 2)(3 4)\nL (1 2 5)"),
            Ok(MovesMap::new(BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2], vec![3, 4]])), ("L".to_owned(), Permutation::new(vec![vec![1, 2, 5]]))]))));
    }

    #[test]
    fn should_skip_empty_lines_and_comments() {
        assert_eq!(MovesMap::from("// test"), Ok(MovesMap::new(BTreeMap::new())));
        assert_eq!(MovesMap::from("\n\n\n"), Ok(MovesMap::new(BTreeMap::new())));
        assert_eq!(MovesMap::from("// first comment\n\n// second comment\n// third comment"),
            Ok(MovesMap::new(BTreeMap::new())));
    }

    #[test]
    fn should_not_parse_moves_map() {
        assert_eq!(MovesMap::from("1 abc").is_err(), true);
        assert_eq!(MovesMap::from("// test\n\nR 1 2").is_err(), true);
    }
}
