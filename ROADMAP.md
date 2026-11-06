# 🗺️ rust-forge Roadmap

**Pace:** each lesson takes about 1–3 focused sessions of 60–90 min. If you finish ~4 lessons a week, you'll reach the capstone in about 8–10 weeks.
**Method:** read the lesson notes and the linked Book sections, predict compiler errors in `playground/`, make the tests pass, then open a PR for review.
**Primary source:** [The Rust Programming Language](https://doc.rust-lang.org/book/) ("the Book"). Chapter numbers below match the current online edition.

> Tick a box only when the lesson's PR is merged with green CI.

---

## Phase 1 · Core language (weeks 1–3)

Goal: write correct single-file Rust and understand ownership well enough that the borrow checker stops feeling random.

- [ ] **L01 · Basics:** variables, mutability, shadowing, scalar & compound types, casting, overflow, functions, expressions, control flow. Book [ch 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [ ] **L02 · Ownership & borrowing:** move vs copy, `&` / `&mut`, borrowing rules, slices, stack vs heap. **The most important lesson.** Book [ch 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [ ] **L03 · Structs, enums & pattern matching:** methods, `impl`, `Option<T>`, `match`, `if let`, `let…else`. Book [ch 5](https://doc.rust-lang.org/book/ch05-00-structs.html), [ch 6](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [ ] **L04 · Collections & text:** `Vec<T>`, `String` vs `&str`, UTF-8, `HashMap`. Book [ch 8](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [ ] **L05 · Error handling:** `panic!` vs `Result`, the `?` operator, custom error enums, `From` conversions. Book [ch 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [ ] 🛠 **P1 · `detlog`:** a CLI that parses detection-event logs (CSV: timestamp, camera, class, confidence, bbox), filters them and prints summary stats. Mirrors Book [ch 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

## Phase 2 · Abstraction (weeks 4–5)

Goal: design reusable, well-tested libraries the way idiomatic Rust crates do.

- [ ] **L06 · Modules, crates & testing:** `mod`, `pub`, `use`, unit vs integration tests, doc tests, workspaces. Book [ch 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [ch 11](https://doc.rust-lang.org/book/ch11-00-testing.html), [ch 14](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
- [ ] **L07 · Generics & traits:** trait bounds, `impl Trait`, derive, standard traits (`Debug`, `Clone`, `From`, `Display`), trait objects `dyn Trait`. Book [ch 10.1–10.2](https://doc.rust-lang.org/book/ch10-00-generics.html), [ch 18.2](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- [ ] **L08 · Lifetimes:** why they exist, elision rules, structs holding references. Book [ch 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [ ] **L09 · Closures & iterators:** `Fn`/`FnMut`/`FnOnce`, iterator adaptors, zero-cost abstractions. Book [ch 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [ ] 🛠 **P2 · `sensorkit`:** a library with a `Sensor` trait, generic filters (moving average, low-pass, 1-D Kalman) and iterator adaptors over readings, fully documented (`cargo doc`) and tested.

## Phase 3 · Systems Rust (weeks 6–8)

Goal: the reasons people choose Rust for robotics and edge AI: fearless concurrency, async I/O and safe interop with C.

- [ ] **L10 · Smart pointers:** `Box`, `Rc`, `RefCell`, `Arc`, `Drop`, interior mutability. Book [ch 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [ ] **L11 · Fearless concurrency:** threads, channels, `Mutex`/`RwLock`, `Send`/`Sync`, scoped threads. Book [ch 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [ ] **L12 · Async Rust:** futures, `async`/`await`, the Tokio runtime, streams, cancellation. Book [ch 17](https://doc.rust-lang.org/book/ch17-00-async-await.html) + [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [ ] **L13 · Patterns & advanced features:** advanced patterns, associated types, newtypes, `macro_rules!`. Book [ch 19](https://doc.rust-lang.org/book/ch19-00-patterns.html), [ch 20](https://doc.rust-lang.org/book/ch20-00-advanced-features.html)
- [ ] **L14 · Unsafe & FFI:** raw pointers, `unsafe` contracts, calling C from Rust (the gateway to CUDA/TensorRT libraries). Book [ch 20.1](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html)
- [ ] 🛠 **P3 · `framepipe`:** a multi-threaded frame-processing pipeline (capture → preprocess → infer (mocked) → sink) with bounded channels, back-pressure, graceful shutdown and latency metrics. You'll build a thread version first, then an async version.

## Phase 4 · Capstone (week 9+) — pick one

We'll check the crate choices against their official docs when we get here, because the ecosystem moves fast.

- [ ] **A · ROS 2 in Rust:** a ROS 2 node in Rust on the Jetson Orin NX (e.g. a LiDAR obstacle monitor), compared against your C++/Python equivalent
- [ ] **B · Video-search API:** an async HTTP service that queries your vLLM (OpenAI-compatible) VLM endpoint and indexes results
- [ ] **C · Native inference:** ONNX model inference from Rust on the Jetson, benchmarked against the Python pipeline

---

## Official references (bookmark these)

| What | Link |
|---|---|
| The Book | https://doc.rust-lang.org/book/ |
| Rust by Example | https://doc.rust-lang.org/rust-by-example/ |
| Standard library docs | https://doc.rust-lang.org/std/ |
| Rustlings (official extra drills) | https://github.com/rust-lang/rustlings |
| The Cargo Book | https://doc.rust-lang.org/cargo/ |
| The Rust Reference | https://doc.rust-lang.org/reference/ |
| Clippy lint list | https://rust-lang.github.io/rust-clippy/master/ |
| Compiler error index | https://doc.rust-lang.org/error_codes/ (or `rustc --explain E0308`) |
