use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub enum BFSResult<T> {
    FoundResults(Vec<Rc<T>>),
    VisitedNodes(HashSet<Rc<T>>)
}

impl<T: Eq + Hash> PartialEq for BFSResult<T> {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Self::FoundResults(results) => match other {
                Self::FoundResults(other_results) => results == other_results,
                Self::VisitedNodes(_) => false
            },
            Self::VisitedNodes(nodes) => match other {
                Self::FoundResults(_) => false,
                Self::VisitedNodes(other_nodes) => {
                    nodes.len() == other_nodes.len() && nodes.iter().all(|e| other_nodes.contains(e))
                }
            }
        }
    }
}

pub fn bfs<T, U, V>(start: Rc<T>, get_nexts: U, is_wanted_node: V, max_results: usize) -> BFSResult<T>
where
    T: Eq + Hash,
    U: Fn(&T) -> Vec<T>,
    V: Fn(&T) -> bool
{
    let mut results: Vec<Rc<T>> = Vec::new();
    let mut queue: VecDeque<Rc<T>> = VecDeque::new();
    let mut visited: HashSet<Rc<T>> = HashSet::new();
    queue.push_back(Rc::clone(&start));
    visited.insert(Rc::clone(&start));

    while let Some(f) = queue.pop_front() {
        if is_wanted_node(&f) {
            results.push(Rc::clone(&f));
            if results.len() == max_results {
                return BFSResult::FoundResults(results);
            }
        }
        // items are of type T
        for item in get_nexts(&f) {
            if !visited.contains(&item) {
                let item_p = Rc::new(item);
                queue.push_back(Rc::clone(&item_p));
                visited.insert(item_p);
            }
        }
    }
    BFSResult::VisitedNodes(visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_start_node() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i] };
        let is_wanted_node = |i: &i32| -> bool { *i == 0 };
        let max_results = 1;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results),
            BFSResult::FoundResults(vec![Rc::new(0)]));
    }

    #[test]
    fn should_find_nothing() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i] };
        let is_wanted_node = |i: &i32| -> bool { *i == 1 };
        let max_results = 1;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results),
            BFSResult::VisitedNodes(HashSet::from([Rc::new(0)])));
    }

    #[test]
    fn should_find_one_end_node() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i + 1] };
        let is_wanted_node = |i: &i32| -> bool { *i == 1 };
        let max_results = 1;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results),
            BFSResult::FoundResults(vec![Rc::new(1)]));
    }

    #[test]
    fn should_find_multiple_end_nodes() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i + 1] };
        let is_wanted_node = |i: &i32| -> bool { *i % 3 == 2 };
        let max_results = 3;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results),
            BFSResult::FoundResults(vec![Rc::new(2), Rc::new(5), Rc::new(8)]));
    }

    #[test]
    fn should_check_visited() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i - 1, *i + 1] };
        let is_wanted_node = |i: &i32| -> bool { *i == 5 };
        let max_results = 1;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results),
            BFSResult::FoundResults(vec![Rc::new(5)]));
    }
}
