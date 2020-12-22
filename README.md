# countdown-solver

Credits for the idea to Matt H from the [youtube video](https://www.youtube.com/watch?v=cVMhkqPP2YI)'s comments about this game. More information can also be found at https://en.wikipedia.org/wiki/Countdown_(game_show)#Numbers_round.

# Current issues

```bash
sort result_memoization_cached | uniq -d -c
```

# Eclipse

```bash
nix shell nixpkgs#rustup nixpkgs#eclipses.eclipse-rust --command eclipse
```

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
RUSTFLAGS="-C target-cpu=native" cargo build --release
valgrind --tool=callgrind ./target/release/countdown-solver > /dev/null
#callgrind_annotate callgrind.out.*
nix run nixpkgs#kcachegrind
```
