# groupname

Group a list of names as evenly as possible.

Example output:
```
[A-E] (size: 1649)
[E-M] (size: 1648)
[M-Z] (size: 1648)
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