# Comms Explorer

A Rust CLI program that finds algorithms to cycle pieces such that:
- The pieces are cycled in a way such that the lengths of the non-trivial cycles match specified target cycle lengths
- For the first permutation matching that spec, the moves given to arrive at that cycle and the resulting elements cycled are both printed out.
- If no permutations match the target cycle lengths, a message is printed out saying that no match was found.

## Running the program

```sh
cargo run -- <moves-filename> <target-cycle-lengths>
```

Moves file format:
```
<move> <cycles>
e.g.
R (1 2 3 4)
```

Note that cycle elements must be positive integers.

Target cycle lengths example:
```sh
"2 2"
```

Output format:
```
<cycles> <moves>
e.g. (1 3)(2 4) R R
```

or a message saying that no algorithm was found.
