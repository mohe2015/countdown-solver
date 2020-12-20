# countdown-solver

# TODO

profile guided optimization

# Rust (without Rustup)

```bash
nix shell nixpkgs#cargo nixpkgs#rustc
RUSTFLAGS="-C target-cpu=native" cargo build --release

```

# Running

```bash
time ./target/release/countdown-solver | tee -a result
```

# Profiling

```bash
nix shell nixpkgs#valgrind
cargo build --release
valgrind --tool=callgrind ./target/release/countdown-solver
#callgrind_annotate callgrind.out.*
nix run nixpkgs#kcachegrind
```
