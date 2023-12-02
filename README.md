# AOC

These are my [Advent of Code](https://adventofcode.com) solutions for 2023.
Feel free to have a look.

## Nix

This is a flake:

```text
├───checks
│   ├───aarch64-darwin
│   │   └───treefmt omitted (use '--all-systems' to show)
│   └───x86_64-linux
│       └───treefmt: derivation 'treefmt-check'
├───devShells
│   ├───aarch64-darwin
│   │   ├───default omitted (use '--all-systems' to show)
│   │   └───pre-commit omitted (use '--all-systems' to show)
│   └───x86_64-linux
│       ├───default: development environment 'dev'
│       └───pre-commit: development environment 'nix-shell'
├───formatter
│   ├───aarch64-darwin omitted (use '--all-systems' to show)
│   └───x86_64-linux: package 'treefmt'
├───hydraJobs
│   ...
└───packages
    ├───aarch64-darwin
    │   ...
    └───x86_64-linux
        ├───day-01: package 'day-01-0.1.0'
        ├───day-02: package 'day-02-0.1.0'
        ...
```
