pub mod permutation;
pub mod algorithm;
pub mod moves_map;
pub mod cycle_lengths;

mod bfs;

use std::rc::Rc;
use std::collections::HashSet;

use algorithm::Algorithm;
use bfs::BFSResult;
use moves_map::MovesMap;
use cycle_lengths::CycleLengths;

#[derive(Debug, PartialEq)]
pub enum AlgorithmResult {
    FoundAlgorithms(Vec<Rc<Algorithm>>),
    FoundCycleLengths(Vec<CycleLengths>)
}

pub fn find_algorithms(moves_map: MovesMap, target_cycle_lengths: CycleLengths, matches_to_find: usize) -> AlgorithmResult {
    let start = Algorithm::identity();
    if target_cycle_lengths == CycleLengths::new(vec![]) {
        return AlgorithmResult::FoundAlgorithms(vec![Rc::new(start)]);
    }
    let get_nexts = |alg: &Algorithm| -> Vec<Algorithm> {
        let mut new_algs: Vec<Algorithm> = Vec::new();
        for (m, p) in moves_map.record.iter() {
            new_algs.push(alg.compose(&m, p));
        }
        new_algs
    };
    let is_wanted_node = |alg: &Algorithm| -> bool {
        let cycle_lengths = alg.permutation.cycles.iter().map(|cycle| cycle.len()).collect::<Vec<usize>>();
        target_cycle_lengths == CycleLengths::new(cycle_lengths)
    };
    let max_results = if matches_to_find > 0 { matches_to_find } else { 1 };
    let bfs_results = bfs::bfs(Rc::new(start), get_nexts, is_wanted_node, max_results);
    match bfs_results {
        BFSResult::FoundResults(results) => {
            // results: Vec<Rc<Algorithm>>
            AlgorithmResult::FoundAlgorithms(results)
        },
        BFSResult::VisitedNodes(visited) => {
            // visited: HashSet<Rc<Algorithm>>
            let mut found_cycle_lengths: HashSet<CycleLengths> = HashSet::new();
            for perm in visited {
                let cycle_lengths = perm.permutation.cycles.iter().map(|cycle| cycle.len()).collect::<Vec<usize>>();
                found_cycle_lengths.insert(CycleLengths::new(cycle_lengths));
            }
            let mut found_cycle_lengths = found_cycle_lengths.into_iter().filter(|l| l.lengths.len() > 0).collect::<Vec<CycleLengths>>();
            found_cycle_lengths.sort();
            AlgorithmResult::FoundCycleLengths(found_cycle_lengths)
        }
    }
}
