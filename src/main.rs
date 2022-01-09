/*use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;*/

use std::env;
use std::fs;
use std::process;

mod bfs;
mod moves_map;
mod permutation;
mod target_cycle_lengths;

use target_cycle_lengths::TargetCycleLengths;

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
    /*
    let moves_map = parse_moves_map(moves_map).unwrap_or_else(|e| {
        println!("Error parsing moves map: {}", e.to_string());
        process:exit(1);
    }); 
    */
    let target_cycle_lengths = TargetCycleLengths::from(&target_cycle_lengths).unwrap_or_else(|e| {
        println!("Error parsing target cycle lengths: {}", e.to_string());
        process::exit(1);
    });

    // do_comms_search(moves_map, target_cycle_lengths.trim());
    // TODO: parse moves_map_file_name into moves_map
    // TODO: impl a get_nexts method for moves_map to send to BFS

}

/*
struct Move {
    label: String
    cycles: Cycles
}
struct TargetCycleLengths(Vec<i32>);
struct AlgResult {
    labels: Vec<&str>;
    cycles: Cycles
}

fn main() {
    // Parse CLI to get the file contents
    
    // Read and parse the contents of each file
    let available_moves = read_file("move-cycle-mappings.txt");

    let target_cycle_lengths = read_file("target-cycle-lengths.txt");

    let available_moves: Vec<Move> = parse_move_cycles(&available_moves);
    println!("Parsed list of available moves:\n{:?}", &available_moves);

    let target_cycle_lengths: TargetCycleLengths = parse_target_cycle_lengths(&target_cycle_lengths);
    println!("Parsed target cycle lengths:\n{:?}", &target_cycle_lengths);

    // Start BFS with the numbers in order, all in 1-cycles (1)...(n)
    // Stop when there is a cycle where the decomposition lengths are the same as the target lengths
    // e.g. if the user wants a 2-2 cycle then the target cycle lengths is "2 2" and the output is
    // "(a b)(c d)\nmove1 ... moveN\n\n"
    // for each move, don't allow for ' notation as well as 1, 2, 3, 4, yet.
    // just repeat moves like RUUUURRRU
    // let results = bfs(Rc::new(start_cycles), get_nexts, is_wanted_node, max_results);
    let results = comm_finder_bfs(available_moves, target_cycle_lengths);

    // Print the results
    for result in results {
        println!("{}", result);
    }
}
*/
