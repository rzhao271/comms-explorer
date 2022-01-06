# Comms Explorer

A Rust CLI program that finds algorithms to cycle pieces such that:
- The pieces are cycled in a way such that the lengths of the cycles match specified target cycle lengths (and the other pieces are in 1-cycles/not moved).
- For each cycle matching that spec, the moves given to arrive at that cycle and the resulting cycles themselves are specified.

## Running the program
cargo run -- moves-file.txt cycles-file.txt

Moves file format:
```
<move> <cycles>
e.g.
R (a b c d)
```

Cycles file format:
```
<target cycle lengths>
e.g.
2 2
```

Output format:
```
<cycles> <moves>
e.g. (a c)(b d) RR
```

