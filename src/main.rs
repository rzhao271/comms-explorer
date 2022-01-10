use std::env;
use std::fs;
use std::process;

use comms_explorer::moves_map::MovesMap;
use comms_explorer::target_cycle_lengths::TargetCycleLengths;

fn main() {
    // Read args
    let mut args = env::args();
    args.next();
    let moves_map_file_name = args.next().unwrap_or_else(|| {
        println!("The first argument should be the moves map file name.");
        process::exit(1);
    });
    let target_cycle_lengths = args.next().unwrap_or_else(|| {
        println!("The second argument should be the target cycle lengths.");
        process::exit(1);
    });
    if args.next().is_some() {
        println!("There should be no additional arguments after the target cycle lengths.");
        process::exit(1);
    }

    let moves_map = fs::read_to_string(&moves_map_file_name).unwrap_or_else(|e| {
        println!("Error reading {}: {}", &moves_map_file_name, e.to_string());
        process::exit(1);
    });
    let moves_map = MovesMap::from(&moves_map).unwrap_or_else(|e| {
        println!("Error parsing moves map: {}", e);
        process::exit(1);
    }); 
    let target_cycle_lengths = TargetCycleLengths::from(&target_cycle_lengths).unwrap_or_else(|e| {
        println!("Error parsing target cycle lengths: {}", e);
        process::exit(1);
    });

    let algorithms = comms_explorer::find_algorithms(moves_map, target_cycle_lengths);
    if algorithms.len() == 0 {
        println!("No algorithm found!");
        return;
    }
    for algorithm in algorithms {
        println!("{}", algorithm);
    }
}

