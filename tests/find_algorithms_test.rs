#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use comms_explorer::find_algorithms;
    use comms_explorer::algorithm::Algorithm;
    use comms_explorer::moves_map::MovesMap;
    use comms_explorer::permutation::Permutation;
    use comms_explorer::target_cycle_lengths::TargetCycleLengths;

    #[test]
    fn should_find_zero_mover() {
        let moves_map = MovesMap { record: BTreeMap::new() };
        let target_cycle_lengths = TargetCycleLengths { target_lengths: vec![0] };
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths), vec![Algorithm::identity()]);
    }

    #[test]
    fn should_find_one_mover() {
        let moves_map = MovesMap { record: BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2]]))]) };
        let target_cycle_lengths = TargetCycleLengths { target_lengths: vec![2] };
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths), vec![Algorithm { permutation: Permutation::new(vec![vec![1, 2]]), moves: "R".to_owned() }]);
    }

    #[test]
    fn should_find_one_mover_double_cycle() {
        let moves_map = MovesMap { record: BTreeMap::from([("R".to_owned(), Permutation::new(vec![vec![1, 2], vec![3, 4, 5]]))]) };
        let target_cycle_lengths = TargetCycleLengths::new(vec![3, 2]);
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths), vec![Algorithm { permutation: Permutation::new(vec![vec![1, 2], vec![3, 4, 5]]), moves: "R".to_owned() }]);
    }

    #[test]
    fn should_find_two_mover() {
        let moves_map = MovesMap { record: BTreeMap::from([("L".to_owned(), Permutation::new(vec![vec![1, 3]])), ("R".to_owned(), Permutation::new(vec![vec![1, 2]]))]) };
        let target_cycle_lengths = TargetCycleLengths { target_lengths: vec![3] };
        assert_eq!(find_algorithms(moves_map, target_cycle_lengths), vec![Algorithm { permutation: Permutation::new(vec![vec![1, 3, 2]]), moves: "L R".to_owned() }]);
    }
}
