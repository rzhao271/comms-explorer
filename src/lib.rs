pub mod permutation;
pub mod algorithm;
pub mod moves_map;
pub mod target_cycle_lengths;

mod bfs;

use std::rc::Rc;

use algorithm::Algorithm;
use moves_map::MovesMap;
use target_cycle_lengths::TargetCycleLengths;

pub fn find_algorithms(moves_map: MovesMap, target_cycle_lengths: TargetCycleLengths) -> Vec<Algorithm> {
    let start = Algorithm::identity();
    if target_cycle_lengths.target_lengths.iter().all(|&len| len == 0) {
        return vec![start];
    }
    let get_nexts = |alg: &Algorithm| -> Vec<Algorithm> {
        let mut new_algs: Vec<Algorithm> = Vec::new();
        for (m, p) in moves_map.record.iter() {
            new_algs.push(alg.compose(&m, p));
        }
        new_algs
    };
    let is_wanted_node = |alg: &Algorithm| -> bool {
        let mut cycle_lengths = alg.permutation.cycles.iter().map(|cycle| { cycle.len() }).collect::<Vec<usize>>();
        cycle_lengths.sort();
        target_cycle_lengths.target_lengths == cycle_lengths
    };
    let max_results = 1; // for now
    bfs::bfs(Rc::new(start), get_nexts, is_wanted_node, max_results)
}
