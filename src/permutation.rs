use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct Permutation {
    cycles: Vec<Vec<i16>>
}

fn traverse(elem: i16, cycles: &Vec<Vec<i16>>) -> i16 {
    let mut e = elem;
    for cycle in cycles {
        if let Some(pos) = cycle.iter().position(|&x| x == e) {
            e = cycle[(pos + 1) % cycle.len()]
        }
    }
    e
}

fn simplify(cycles: Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    // Collect all the elements of all the cycles, 
    // and then sort them into a heap.
    let mut b: BinaryHeap<i16> = BinaryHeap::new();
    let mut visited: HashSet<i16> = HashSet::new();
    for cycle in &cycles {
        for &e in cycle {
            visited.insert(e);
        }
    }
    for &e in &visited {
        b.push(-e);
    }

    // Now find the new cycles traced by 
    // the popped elements.
    let mut new_cycles: Vec<Vec<i16>> = Vec::new();
    visited.clear();
    while let Some(mut elem) = b.pop() {
        elem = -elem;
        if visited.contains(&elem) {
            continue;
        }
        visited.insert(elem);
        let first_elem = elem;
        let mut new_cycle: Vec<i16> = vec![elem]; 

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

impl Permutation {
    pub fn new(cycles: Vec<Vec<i16>>) -> Permutation {
        Permutation {
            cycles: simplify(cycles)
        }
    }

    pub fn compose(&self, other: &Permutation) -> Permutation {
        let mut v: Vec<Vec<i16>> = Vec::new();
        self.cycles.iter().for_each(|x| v.push(x.to_vec()));
        other.cycles.iter().for_each(|x| v.push(x.to_vec()));
        Permutation {
            cycles: simplify(v)
        }
    }
}

impl PartialEq for Permutation {
    fn eq(&self, other: &Permutation) -> bool {
        (self.cycles.len() == 0 && other.cycles.len() == 0) ||
        (self.cycles == other.cycles)
    } 
    fn ne(&self, other: &Permutation) -> bool {
        !self.eq(other)
    }
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for v in &self.cycles {
            write!(f, "({})", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::permutation::*;

    #[test]
    fn should_print_formatted_cycles() {
        assert_eq!(Permutation::new(vec![]).to_string(), "");
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
        assert_eq!(Permutation::new(vec![]), Permutation::new(vec![vec![1]]));
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
    }

    #[test]
    fn should_compose() {
        // assert_eq!(); 
    }
}

