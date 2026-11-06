# 🦀 rust-forge

[![CI](https://github.com/KaveeshwaraBandara/rust-forge/actions/workflows/ci.yml/badge.svg)](https://github.com/KaveeshwaraBandara/rust-forge/actions/workflows/ci.yml)

My hands-on path from zero to production Rust: test-driven lessons, mini-projects and a robotics/edge-AI capstone on the Jetson Orin NX.

**Rules of this repo:** tests define "done", clippy is law, and every lesson ships as a reviewed PR.

---

## Layout

```text
rust-forge/
├── lessons/          # one crate per lesson: notes (README) + exercises (src/lib.rs) + tests
│   └── l01_basics/
├── projects/         # one crate per mini-project (P1–P3) and the capstone
├── playground/       # scratch crate for compiler drills and experiments
├── docs/WORKFLOW.md  # how I work: branches, commits, PRs, review
├── ROADMAP.md        # full curriculum with links to the official docs
└── LEARNING_LOG.md   # what I learned, what confused me, concept-check answers
```

## Quick start

First time only (needs `gh auth login`): this creates the git history, the GitHub repo, labels, milestones and one issue per lesson.

```bash
bash scripts/bootstrap.sh            # or: VISIBILITY=private bash scripts/bootstrap.sh
```

Daily commands:

```bash
rustup update stable                 # rust-toolchain.toml pins stable + rustfmt + clippy
cargo test -p l01_basics             # run one lesson's tests (they fail until you solve them)
cargo test -p l01_basics e05         # run only the tests whose name contains "e05"
cargo run  -p playground             # run your scratchpad
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
```

## The loop (every lesson)

1. Take the lesson's GitHub Issue and check out its branch: `git switch lesson/NN-topic`
2. Read `lessons/lNN_*/README.md` and the linked Book sections
3. Do the **compiler drills** in `playground/`: predict the error first, then run
4. Replace each `todo!()` in `src/lib.rs` until `cargo test -p lNN_*` is green
5. Get clippy and fmt clean, and answer the concept check in `LEARNING_LOG.md`
6. Push, open a PR (`Closes #N`), ask for review, merge when CI is green ✅

Details are in [docs/WORKFLOW.md](docs/WORKFLOW.md).

## Progress

| # | Lesson / Project | Status |
|---|---|---|
| L01 | Basics: variables, types, functions, control flow | 🟡 in progress |
| L02 | Ownership & borrowing | ⬜ |
| L03 | Structs, enums & pattern matching | ⬜ |
| L04 | Collections & text | ⬜ |
| L05 | Error handling | ⬜ |
| P1 | `detlog`: detection-log CLI | ⬜ |
| L06 | Modules, crates & testing | ⬜ |
| L07 | Generics & traits | ⬜ |
| L08 | Lifetimes | ⬜ |
| L09 | Closures & iterators | ⬜ |
| P2 | `sensorkit`: sensor-filter library | ⬜ |
| L10 | Smart pointers | ⬜ |
| L11 | Fearless concurrency | ⬜ |
| L12 | Async Rust | ⬜ |
| L13 | Patterns & advanced features | ⬜ |
| L14 | Unsafe & FFI | ⬜ |
| P3 | `framepipe`: concurrent frame pipeline | ⬜ |
| 🎯 | Capstone (ROS 2 / VLM API / ONNX on Jetson) | ⬜ |

Full curriculum: [ROADMAP.md](ROADMAP.md)

## License

MIT
