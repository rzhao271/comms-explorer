# Comms Explorer

A Rust CLI program that finds algorithms to cycle pieces such that:
- The pieces are cycled in a way such that the lengths of the non-trivial cycles match specified target cycle lengths
- For the first permutation matching that spec, the moves given to arrive at that cycle and the resulting elements cycled are both printed out.
- If no permutations match the target cycle lengths, a message is printed out saying that no match was found.

## Running the program

```sh
./comms-explorer <moves-filename> <target-cycle-lengths> [<matches-to-find>]
```

Where `<moves-filename>` is the filename of a plain text file,
where each line of the plain text file is one of the following:
- A comment starting with `//`
- A string representing the label given to a move, followed by a space, followed by the cycle elements
  that are cycled that move. Cycle elements must be positive integers.

Where `<target-cycle-lengths>` is a string with a series of positive integers,
and where `<matches-to-find>` is an optional parameter, but when it shows up,
it is a positive integer. Otherwise, it defaults to `1`.

The program will try to find `<matches-to-find>` combination(s) of moves
from `<moves-filename>` such that the lengths of the resulting permutation cycles
match the lengths given by `<target-cycle-lengths>`.
One-cycles are excluded from consideration.

If no algorithm is found, the program prints out the distinct cycle lengths of all found permutations
in lexicographical order. Otherwise, the program prints out at most `<matches-to-find>` algorithms.

### Sample run

```sh
$ cat bicube-fuse.txt
A (1 2 3 5 4)
B (5 6 1 3 2)
C (3 4 5 1 6)
$ ./comms-explorer bicube-fuse.txt "2 2"
Found algorithms:
(1 6)(2 4) A A A B
$ ./comms-explorer bicube-fuse.txt "3"
No algorithms found. Found cycle lengths:
2 2
3 3
5
```
