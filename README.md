# groupname

Group a list of names as evenly as possible.

Example output:
```
[A-H] (8 letters, 16 occurrences)
[I-Q] (9 letters, 15 occurrences)
[R-Z] (10 letters, 16 occurrences)
```

## Usage

To build and run, use `cargo run --release -- names.txt 3`. Or, run a compiled binary with `groupname names.txt`.

Use `--length n` for lengths longer than 1. For example, `--length 2` would produce buckets like `[Ic-Qu]`.

## TODO

- Piping support
- Design/document a library
- Parse CSV and other formats
- Unit tests
- "Exhaustive" result with 'A' and 'Z' as the first and last lettters.