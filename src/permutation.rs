use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Permutation {
    pub cycles: Vec<Vec<u16>>
}

fn traverse(elem: u16, cycles: &Vec<Vec<u16>>) -> u16 {
    let mut e = elem;
    for cycle in cycles {
        if let Some(pos) = cycle.iter().position(|&x| x == e) {
            e = cycle[(pos + 1) % cycle.len()]
        }
    }
    e
}

fn simplify(cycles: Vec<Vec<u16>>) -> Vec<Vec<u16>> {
    // Collect all the elements of all the cycles, 
    // and then sort them into a heap.
    let mut b: BinaryHeap<u16> = BinaryHeap::new();
    let mut visited: HashSet<u16> = HashSet::new();
    for cycle in &cycles {
        for &e in cycle {
            visited.insert(e);
        }
    }
    for &e in &visited {
        b.push(u16::MAX - e);
    }

    // Now find the new cycles traced by 
    // the popped elements.
    let mut new_cycles: Vec<Vec<u16>> = Vec::new();
    visited.clear();
    while let Some(mut elem) = b.pop() {
        elem = u16::MAX - elem;
        if visited.contains(&elem) {
            continue;
        }
        visited.insert(elem);
        let first_elem = elem;
        let mut new_cycle: Vec<u16> = vec![elem]; 

        elem = traverse(elem, &cycles);
        while elem != first_elem {
            visited.insert(elem);
            new_cycle.push(elem);
            elem = traverse(elem, &cycles);
        }
        if new_cycle.len() >= 2 {
            new_cycles.push(new_cycle);
        }
    }
    new_cycles
}

// Example: "(1 2 3)(4 5)" -> !vec["1 2 3", "4 5"]
fn extract_cycles(s: &str) -> Result<Vec<&str>, String> {
    fn extract_cycles_acc<'a>(s: &'a str, mut acc: Vec<&'a str>) -> Result<Vec<&'a str>, String> {
        if s.trim().len() == 0 {
            return Ok(acc);
        }
        let start_bracket_ind = s.find('(');
        if start_bracket_ind.is_none() {
            return Err("'(' not found even though there were characters after.".to_owned());
        }
        let start_bracket_ind = start_bracket_ind.unwrap();
        let end_bracket_ind = s.find(')');
        if end_bracket_ind.is_none() {
            return Err("')' not found even though there was '(' character before.".to_owned());
        }
        let end_bracket_ind = end_bracket_ind.unwrap();
        if start_bracket_ind > end_bracket_ind {
            return Err("')' found before '(' when it is supposed to be after.".to_owned());
        }
        acc.push(&s[start_bracket_ind + 1..end_bracket_ind]);
        extract_cycles_acc(&s[end_bracket_ind + 1..], acc)
    }
    extract_cycles_acc(s, Vec::<&str>::new())
}

impl Permutation {
    pub fn new(cycles: Vec<Vec<u16>>) -> Permutation {
        Permutation {
            cycles: simplify(cycles)
        }
    }

    pub fn from(s: &str) -> Result<Permutation, String> {
        let mut cycles: Vec<Vec<u16>> = Vec::new();
        match extract_cycles(&s) {
            Ok(extracted_cycles) => {
                for extracted_cycle in extracted_cycles {
                    let mut cycle: Vec<u16> = Vec::new();
                    for num in extracted_cycle.trim().split(" ") {
                        if num == "" {
                            continue;
                        }
                        if let Ok(num) = num.parse::<u16>() {
                            cycle.push(num);
                        } else {
                            return Err(format!("Invalid cycle element '{}' in permutation string '{}'.", &num, &s));
                        }
                    }
                    cycles.push(cycle);
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
        Ok(Permutation::new(cycles))
    }

    pub fn compose(&self, other: &Permutation) -> Permutation {
        let mut v: Vec<Vec<u16>> = Vec::new();
        self.cycles.iter().for_each(|x| v.push(x.to_vec()));
        other.cycles.iter().for_each(|x| v.push(x.to_vec()));
        Permutation::new(v)
    }
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.cycles.len() == 0 {
            return write!(f, "[id]");
        }
        for c in &self.cycles {
            write!(f, "({})", c.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_print_formatted_cycles() {
        assert_eq!(Permutation::new(vec![]).to_string(), "[id]");
        assert_eq!(Permutation::new(vec![vec![1, 2]]).to_string(), "(1 2)");
        assert_eq!(Permutation::new(vec![vec![2, 1]]).to_string(), "(1 2)");
        assert_eq!(Permutation::new(vec![vec![1, 2, 3], vec![4, 5, 6]]).to_string(), "(1 2 3)(4 5 6)");
        assert_eq!(Permutation::new(vec![vec![4, 5, 6], vec![3, 2, 1]]).to_string(), "(1 3 2)(4 5 6)");
    }

    #[test]
    fn should_have_equal_cycles() {
        assert_eq!(Permutation::new(vec![]), Permutation::new(vec![]));
        assert_eq!(Permutation::new(vec![vec![1]]), Permutation::new(vec![vec![1]]));
        assert_eq!(Permutation::new(vec![vec![1, 2], vec![3]]), Permutation::new(vec![vec![1, 2], vec![3]]));
        assert_ne!(Permutation::new(vec![vec![1]]), Permutation::new(vec![vec![1, 2]]));
        assert_ne!(Permutation::new(vec![vec![1, 2], vec![3, 4]]), Permutation::new(vec![vec![1, 2], vec![3, 5]]));
    }

    #[test]
    fn should_simplify() {
        assert_eq!(Permutation::new(vec![]), Permutation::new(vec![]));
        assert_eq!(Permutation::new(vec![vec![1]]), Permutation::new(vec![vec![1]]));
        assert_eq!(Permutation::new(vec![vec![1, 2]]), Permutation::new(vec![vec![1, 2]]));
        assert_eq!(Permutation::new(vec![vec![1], vec![2]]), Permutation::new(vec![]));
        assert_eq!(Permutation::new(vec![vec![1, 2], vec![1, 2]]), Permutation::new(vec![]));
        assert_eq!(Permutation::new(vec![vec![1, 2], vec![3, 2]]), Permutation::new(vec![vec![1, 3, 2]]));
        assert_eq!(Permutation::new(vec![vec![1, 2], vec![3, 4]]), Permutation::new(vec![vec![1, 2], vec![3, 4]]));
        assert_eq!(Permutation::new(vec![vec![1, 2], vec![3, 4]]), Permutation::new(vec![vec![2, 1], vec![4, 3]]));
    }

    #[test]
    fn should_compose() {
        assert_eq!(Permutation::new(vec![vec![1, 2]]).compose(&Permutation::new(vec![vec![3, 2]])),
            Permutation::new(vec![vec![1, 3, 2]]));
        assert_eq!(Permutation::new(vec![vec![1, 2]]).compose(&Permutation::new(vec![])),
            Permutation::new(vec![vec![1, 2]]));
        assert_eq!(Permutation::new(vec![]).compose(&Permutation::new(vec![vec![1, 2]])),
            Permutation::new(vec![vec![1, 2]]));
        assert_eq!(Permutation::new(vec![vec![1]]).compose(&Permutation::new(vec![vec![2]])),
            Permutation::new(vec![]));
        assert_eq!(Permutation::new(vec![vec![1, 2]]).compose(&Permutation::new(vec![vec![1, 2]])),
            Permutation::new(vec![]));
        assert_eq!(Permutation::new(vec![vec![1, 2], vec![3, 4]]).compose(&Permutation::new(vec![vec![1, 2, 3]])),
            Permutation::new(vec![vec![1, 3, 4]]));
    }

    #[test]
    fn should_parse() {
        assert_eq!(Permutation::from(""), Ok(Permutation::new(vec![])));
        assert_eq!(Permutation::from("(1)"), Ok(Permutation::new(vec![])));
        assert_eq!(Permutation::from("(1 2)"), Ok(Permutation::new(vec![vec![1, 2]])));
        assert_eq!(Permutation::from("(1 2 3)(4 5)"), Ok(Permutation::new(vec![vec![1, 2, 3], vec![4, 5]])));
        assert_eq!(Permutation::from("( 1 2  3)   (   4    5    )"), Ok(Permutation::new(vec![vec![1, 2, 3], vec![4, 5]])));
        assert_eq!(Permutation::from("((1 2))").is_err(), true);
        assert_eq!(Permutation::from("(1 2").is_err(), true);
        assert_eq!(Permutation::from("(1 3) 2").is_err(), true);
        assert_eq!(Permutation::from("1 )2").is_err(), true);
        assert_eq!(Permutation::from(")1 2(").is_err(), true);
    }
}

