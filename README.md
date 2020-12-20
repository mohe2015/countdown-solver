# countdown-solver

# TODO

profile guided optimization

# Profiling

```bash
nix shell nixpkgs#valgrind
cargo build --release
valgrind --tool=callgrind ./target/release/countdown-solver
#callgrind_annotate callgrind.out.*
nix run nixpkgs#kcachegrind
```