/*use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;*/

mod bfs;
mod permutation;

fn main() {
}

/*
fn read_file(path_name: &str) -> String {
    // Take in a list of move-cycle mappings
    let path = Path::new(path_name);
    let path_display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Unable to open {}: {}", path_display, why),
        Ok(file) => file
    };
    let mut move_cycles = String::new();
    if let Err(why) = file.read_to_string(&mut move_cycles) {
        panic!("Cannot read {}: {}", path_display, why);
    }
    move_cycles 
}

// Cycles = list of list of uints
// This way we don't need to mess with strings
type Cycle = Vec<u32>;
type Cycles = Vec<Cycle>;

let global_cycle_element_symbols: Vec<String> = Vec::new();

fn initialize_global_symbols(s: &str) {
}

fn parse_cycles(s: &str) -> Cycles {
    // keep looking for '(', ' ', and ')'
    for b in s.chars() {
        match b {
            '(' => {
            }
            ' ' => {
            } 
            ')' => {
            }
            c => {
                println!("Do something with {}", &c);
            }
        }
    }
    Vec::new()
}


fn parse_move_cycles(s: &str) -> HashMap<String, Cycles> {
    // For each line, the first word is the move, and after that, parse the cycles
    let mut move_cycles: HashMap<String, Cycles> = HashMap::new();

    let lines = s.split("\n");
    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        let cube_move: &str = words.get(0).expect("Invalid move-cycle mapping");
        let rest_str = &words[1..].concat();
        let cycles = parse_cycles(&rest_str);
        move_cycles.insert(cube_move.to_string(), cycles);
    }
    move_cycles    
}
 
fn parse_target_cycle_lengths(s: &str) -> Vec<i32> {
    let results: Vec<Result<i32, _>> = s.trim().split(' ').map(|s| s.parse()).collect();
    let mut target_cycle_lengths: Vec<i32> = Vec::new();
    for result in results {
        match result {
            Err(why) => panic!("Invalid target cycle length: {}", why),
            Ok(target_cycle_length) => target_cycle_lengths.push(target_cycle_length)
        }
    }
    target_cycle_lengths
}

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
