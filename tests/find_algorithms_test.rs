#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::rc::Rc;

    use comms_explorer::AlgorithmResult;
    use comms_explorer::find_algorithms;
    use comms_explorer::algorithm::Algorithm;
    use comms_explorer::moves_map::MovesMap;
    use comms_explorer::permutation::Permutation;
    use comms_explorer::cycle_lengths::CycleLengths;

    #[test]
    fn should_find_zero_mover() {
        let moves_map = MovesMap::new(BTreeMap::new());
        let target_cycle_lengths = CycleLengths::new(vec![0]);
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths, 1),
            AlgorithmResult::FoundAlgorithms(vec![Rc::new(Algorithm::identity())]));
    }

    #[test]
    fn should_find_one_mover() {
        let moves_map = MovesMap::new(BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2]]))]));
        let target_cycle_lengths = CycleLengths::new(vec![2]);
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths, 1),
            AlgorithmResult::FoundAlgorithms(vec![Rc::new(Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R".to_owned() })]));
    }

    #[test]
    fn should_find_one_mover_double_cycle() {
        let moves_map = MovesMap::new(BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2], vec![3, 4, 5]]))]));
        let target_cycle_lengths = CycleLengths::new(vec![3, 2]);
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths, 1),
            AlgorithmResult::FoundAlgorithms(vec![Rc::new(Algorithm { permutation: Permutation::new(vec![vec![1, 2], vec![3, 4, 5]]), moves: "R".to_owned() })]));
    }

    #[test]
    fn should_find_two_mover() {
        let moves_map = MovesMap::new(BTreeMap::from([("L".to_owned(), Permutation::new(vec![vec![1, 3]])), ("R".to_owned(), Permutation::new(vec![vec![1, 2]]))]));
        let target_cycle_lengths = CycleLengths::new(vec![3]);
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths, 1),
            AlgorithmResult::FoundAlgorithms(vec![Rc::new(Algorithm { permutation: Permutation::new(vec![vec![1, 3, 2]]), moves: "L R".to_owned() })]));
    }

    #[test]
    fn should_not_find() {
        let moves_map = MovesMap::new(BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2, 3, 4]]))]));
        let target_cycle_lengths = CycleLengths::new(vec![3]);
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths, 1),
            AlgorithmResult::FoundCycleLengths(vec![CycleLengths::new(vec![2, 2]), CycleLengths::new(vec![4])]));
    }

    #[test]
    fn should_find_one_request_two() {
        let moves_map = MovesMap::new(BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2]]))]));
        let target_cycle_lengths = CycleLengths::new(vec![2]);
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths, 2),
            AlgorithmResult::FoundAlgorithms(vec![Rc::new(Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R".to_owned() })]));
    }
}
