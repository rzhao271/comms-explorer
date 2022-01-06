use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

pub fn bfs<T, U, V>(start: Rc<T>, get_nexts: U, is_wanted_node: V, max_results: usize) -> Vec<T> 
where 
    T: Clone + Eq + Hash,
    U: Fn(&T) -> Vec<T>,
    V: Fn(&T) -> bool
{
    let mut results: Vec<T> = Vec::new();
    let mut queue: VecDeque<Rc<T>> = VecDeque::new();
    let mut visited: HashSet<Rc<T>> = HashSet::new();
    queue.push_back(Rc::clone(&start));
    visited.insert(Rc::clone(&start));

    while let Some(f) = queue.pop_front() {
        if is_wanted_node(&f) {
            results.push((*f).clone());
            if results.len() == max_results {
                return results;
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
    results
}

#[cfg(test)]
mod test {
    use crate::bfs::*;

    #[test]
    fn should_find_start_node() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i] };
        let is_wanted_node = |i: &i32| -> bool { *i == 0 };
        let max_results = 1;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results), vec![0]);
    }

    #[test]
    fn should_find_nothing() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i] };
        let is_wanted_node = |i: &i32| -> bool { *i == 1 };
        let max_results = 1;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results), vec![]);
    }

    #[test]
    fn should_find_one_end_node() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i + 1] };
        let is_wanted_node = |i: &i32| -> bool { *i == 1 };
        let max_results = 1;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results), vec![1]);
    }

    #[test]
    fn should_find_multiple_end_nodes() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i + 1] };
        let is_wanted_node = |i: &i32| -> bool { *i % 3 == 2 };
        let max_results = 3;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results), vec![2, 5, 8]);
    }

    #[test]
    fn should_check_visited() {
        let start = 0;
        let get_nexts = |i: &i32| -> Vec<i32> { vec![*i - 1, *i + 1] };
        let is_wanted_node = |i: &i32| -> bool { *i == 5 };
        let max_results = 1;
        assert_eq!(bfs(Rc::new(start), get_nexts, is_wanted_node, max_results), vec![5]);
    }
}
