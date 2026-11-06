# L01 · Rust Basics: variables, types, functions, control flow

| | |
|---|---|
| **Time** | 2 sessions (~2–3 h total) |
| **Book** | [3.1 Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) · [3.2 Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) · [3.3 Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) · [3.5 Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) |
| **Goal** | Write small, correct Rust functions and read compiler errors fluently |
| **Done when** | `cargo test -p l01_basics` is green, clippy and fmt are clean, the concept check is answered, and the PR is merged |

---

## 0. Setup check (10 min)

```bash
rustup update stable
rustc --version && cargo --version     # any 1.85+ works; newer is better
cargo test -p l01_basics               # expect all 16 tests to fail (12 exercises). That's correct for now.
```

The cargo commands you'll use every day:

| Command | What it does |
|---|---|
| `cargo check` | Type-check only, no binary. Fastest feedback, so run it constantly. |
| `cargo build` / `cargo run` | Compile (debug profile) / compile and run |
| `cargo build --release` | Optimized build. Use it for benchmarks and the Jetson. |
| `cargo test -p l01_basics e04` | Run tests in one crate whose name contains `e04` |
| `cargo test ... -- --nocapture` | Show `println!` output from tests |
| `cargo clippy` | Linter. It teaches idiomatic Rust, so read every suggestion. |
| `cargo fmt` | Auto-format. Never argue about style. |
| `cargo doc --open` | Generate and open HTML docs for your crate and dependencies |
| `rustc --explain E0308` | Long explanation of any error code |

---

## 1. The mental shift (coming from Python and C++)

| | Python | C++ | **Rust** |
|---|---|---|---|
| Typing | dynamic | static | static **+ inference** |
| Memory | GC / refcount | manual + RAII | **ownership** checked at compile time, RAII, no GC |
| Default mutability | mutable | mutable | **immutable** |
| "No value" | `None` (anywhere) | `nullptr` (anywhere) | **no null**: `Option<T>` makes absence explicit in the type |
| Errors | exceptions | exceptions | **no exceptions**: `Result<T, E>`, plus `panic!` for bugs |
| Truthiness | `if 3:` ok | `if (3)` ok | **only `bool`** in conditions |
| Int overflow | bigints | signed = UB | **panic in debug, wrap in release**, or choose explicitly |
| Out-of-bounds index | `IndexError` | UB | **panic** (never silent memory corruption) |

The theme is that Rust moves as many bugs as possible from runtime to compile time. The compiler is strict, but it is also the best tutor you'll have. **Read whole error messages, including the `help:` lines.**

---

## 2. Variables, mutability, shadowing, constants

```rust
let speed = 1.5;           // immutable binding; type inferred as f64
// speed = 2.0;            // ❌ E0384: cannot assign twice to immutable variable

let mut odom_x = 0.0;      // mutability is opt-in and visible at the declaration
odom_x += 0.1;

const MAX_SPEED_MPS: f64 = 1.2;   // compile-time constant: type REQUIRED, SCREAMING_CASE
```

**Shadowing** declares a *new* variable with the same name, and it may have a different type:

```rust
let reading = "42";                         // &str (text from a serial port, say)
let reading: i32 = reading.parse().unwrap(); // new binding, new type; the old one is gone
```

| | `let mut x` | shadowing `let x = …` again |
|---|---|---|
| Same variable? | yes, mutated in place | no, a brand-new binding |
| Type can change? | ❌ | ✅ |
| Typical use | accumulators, state | transforming a value step by step (parse → validate → convert) |

> **Why immutable by default?** If something can't change, you never have to ask who changed it. In concurrent code (L11) this becomes a superpower.

---

## 3. Types

### Scalars

| Kind | Types | Notes |
|---|---|---|
| Signed int | `i8 i16 i32 i64 i128 isize` | default is `i32` |
| Unsigned int | `u8 u16 u32 u64 u128 usize` | `u8` for bytes and pixels; **`usize` for lengths and indexes** |
| Float | `f32 f64` | default is `f64`; `f32` is common on embedded/GPU |
| Bool | `bool` | |
| Char | `char` | **4 bytes**: a Unicode scalar value, not a byte |

Literals: `1_000_000`, `0xFF`, `0b1010_0000`, `255u8`, `2.5_f32`.

### No implicit conversions, ever

```rust
let ticks: i32 = 5;
let total: i64 = ticks;          // ❌ E0308 mismatched types
let total: i64 = ticks.into();   // ✅ lossless (From/Into), and the compiler proves it can't fail
let total = i64::from(ticks);    // ✅ same thing, more explicit
```

`as` is a **sharp knife**. It always compiles and silently changes values:

```rust
300_i32 as u8      // 44: keeps the low 8 bits (300 = 0x12C → 0x2C)
-1_i32  as u32     // 4294967295: the bits are reinterpreted
3.99_f64 as i32    // 3: truncates toward zero
1e10_f64 as i32    // 2147483647: float→int saturates
f64::NAN as i32    // 0
```

When a conversion can fail, use `TryFrom`, which makes you handle failure:

```rust
u8::try_from(300_i32)   // Err(..)
u8::try_from(200_i32)   // Ok(200)
```

### Integer overflow: you choose the semantics

```rust
fn add_one(x: u8) -> u8 { x + 1 }
add_one(255)   // debug build: panics "attempt to add with overflow"
               // release build: wraps to 0 (overflow checks are off by default in release)
```

When overflow is *expected*, say so explicitly:

| Method | `250u8 + 10` gives | Use when |
|---|---|---|
| `wrapping_add` | `4` | counters that roll over (encoders, sequence numbers, timestamps) |
| `checked_add` | `None` | overflow means bad input, so handle it |
| `saturating_add` | `255` | clamp at the limits (PWM, brightness, audio) |
| `overflowing_add` | `(4, true)` | you need the value *and* a flag |

> 🤖 **Robotics tie-in:** wheel encoder counters are usually `u16`/`i16` registers that wrap. If you compute `now - prev` naively you get a huge garbage delta at rollover. `wrapping_sub` plus a cast gives the correct signed delta *through* the wrap. That's exercise E04.

### Compound types

```rust
let pose: (f64, f64, f64) = (1.0, 2.0, 0.5);  // tuple: fixed size, mixed types
let (x, y, theta) = pose;                      // destructuring
let heading = pose.2;                          // field access by index

let ranges: [f32; 8] = [0.0; 8];               // array: fixed size; the LENGTH IS PART OF THE TYPE
let n = ranges.len();                          // 8 (a usize)
// ranges[i] with a bad i → panic "index out of bounds", never UB
```

`()` (the "unit" type, an empty tuple) is what functions return when they return "nothing". It's Rust's `void`, except it's a real value.

---

## 4. Functions: statements vs expressions

```rust
fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0      // ← no semicolon: this is the return value
}
```

- **Parameter and return types are mandatory.** Signatures are contracts. Inference works *inside* a function but never across its boundary, so changing a function's body can't silently change its callers.
- The **last expression** of a block is its value. Adding `;` turns it into a **statement**, which evaluates to `()`:

```rust
fn double(x: i32) -> i32 {
    x * 2;          // ❌ E0308 mismatched types: expected `i32`, found `()`
}                   //    help: remove this semicolon to return this value
```

- `return` is for **early** exits. Idiomatic Rust doesn't use it on the last line.
- Blocks are expressions too:

```rust
let clearance = {
    let raw = 0.42;
    raw - 0.05          // the block evaluates to this
};
```

Naming: `snake_case` for functions and variables, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants. The compiler warns if you break this.

---

## 5. Control flow: almost everything is an expression

### `if` is an expression, and conditions must be `bool`

```rust
let mode = if battery_v > 3.6 { "normal" } else { "eco" };   // like a ternary
// if obstacles { … }           // ❌ E0308: expected `bool`, found integer
// if obstacles > 0 { … }       // ✅ say what you mean
```

Both branches must have the **same type** (E0308 "`if` and `else` have incompatible types").

### `loop` (can return a value) / `while` / `for`

```rust
let mut attempts = 0;
let result = loop {
    attempts += 1;
    if attempts == 3 {
        break attempts * 10;   // `break` with a value: the loop evaluates to 30
    }
};

'outer: for row in 0..480 {          // labeled loop
    for col in 0..640 {
        if row * col > 1000 { break 'outer; }
    }
}

while battery_mv > 3300 { battery_mv -= 50; }

for i in 0..5 { }            // 0,1,2,3,4      (exclusive)
for i in 0..=5 { }           // 0,1,2,3,4,5    (inclusive)
for i in (0..10).rev().step_by(3) { }   // 9,6,3,0
for r in ranges { }                     // iterate an array's values
for (i, r) in ranges.iter().enumerate() { }  // index + reference to the value
```

> Prefer `for x in collection` over `for i in 0..collection.len()`. It's clearer and skips bounds checks, and clippy will nag you otherwise (`needless_range_loop`).

### `match` preview (full treatment in L03)

```rust
let label = match millivolts {
    0..=3299    => "critical",
    3300..=3599 => "low",
    _           => "ok",       // match must be EXHAUSTIVE: `_` catches the rest
};
```

> 💡 Integer millivolts instead of float volts give exact comparisons with no `3.5999999` surprises. Choose units deliberately.

### `Option<T>` sneak peek

A function that might not have an answer returns `Option<T>`, which is either `Some(value)` or `None`:

```rust
fn first_even(xs: [i32; 4]) -> Option<usize> {
    for (i, x) in xs.iter().enumerate() {
        if x % 2 == 0 { return Some(i); }
    }
    None
}
```

That's all you need for E09 and E12. L03 covers the rest.

---

## 6. Printing and debugging

```rust
let v = 3.14159; let name = "lidar";
println!("{v}");             // inline variable (Display)
println!("{v:.2}");          // 3.14
println!("[{v:>8.2}]");      // [    3.14]  right-aligned, width 8
println!("{name:<8}|");      // left-aligned
println!("{:?}", (1, 2.5));  // Debug format, works for tuples, arrays, etc.
println!("{:#?}", (1, 2.5)); // pretty Debug (multi-line)
eprintln!("to stderr");
let s = format!("v={v:.1}"); // same syntax, returns a String
let x = dbg!(2 * 21);        // prints "[src/main.rs:L:C] 2 * 21 = 42" to stderr AND returns 42
```

---

## 7. 🔧 Compiler drills (do these in `playground/`)

For each one, paste it into `playground/src/main.rs`, **write down the error you expect**, then run `cargo run -p playground`. Read the full message, fix it, and move on.

```rust
// D1: what error code? what does the help: line suggest?
let speed = 1.0;
speed = 2.0;

// D2: what conversion does the compiler suggest?
let ticks: i32 = 5;
let total: i64 = ticks;

// D3: no truthiness. Fix it so it means "if there are obstacles".
let obstacles = 3;
if obstacles { println!("stop!"); }

// D4: put this fn OUTSIDE main. Why is the error on the signature line?
fn double(x: i32) -> i32 { x * 2; }

// D5: this one isn't a type error but a deny-by-default LINT. Which one?
let pwm: u8 = 256;

// D6: what are the two branch types?
let battery_ok = true;
let mode = if battery_ok { "normal" } else { 0 };

// D7: this compiles. What happens at runtime?
let ranges = [1.2, 0.8, 3.1];
let i = std::env::args().count() + 4;   // index known only at runtime
println!("{}", ranges[i]);

// D8: run it twice: `cargo run -p playground` and `cargo run -p playground --release`.
//     Explain the difference.
fn add_one(x: u8) -> u8 { x + 1 }          // (outside main)
let n: u8 = std::env::args().count() as u8 + 254;   // 255 at runtime
println!("{}", add_one(n));
```

Then run `rustc --explain E0384` and `rustc --explain E0308` once each.

---

## 8. 🏋️ Exercises: `src/lib.rs`

Every function body is `todo!()`. Replace it until its tests pass. **Don't edit the tests.**

| # | Function | Practices |
|---|---|---|
| E01 | `celsius_to_fahrenheit` | expression bodies, `f64` |
| E02 | `wheel_linear_speed` | constants from `std`, unit reasoning |
| E03 | `to_pwm` | safe narrowing (`i32` → `u8`) without `as` surprises |
| E04 | `encoder_delta` | **wrapping arithmetic** on rolling counters |
| E05 | `battery_status` | `match` on integer ranges, `&'static str` |
| E06 | `sum_of_multiples` | `for` + ranges + `if` |
| E07 | `average` | arrays, iteration, `as` for `usize` → `f64` |
| E08 | `max_reading` | tuples, manual loop, tie-breaking |
| E09 | `first_obstacle` | early `return`, `Option` preview |
| E10 | `collatz_steps` | `while` |
| E11 | `count_digits` | `loop` with `break value` |
| E12 ⭐ | `checked_factorial` | overflow-aware math with `checked_mul` + `Option` |

```bash
cargo test -p l01_basics            # all
cargo test -p l01_basics e04        # just one exercise
```

Rules:
1. Make one exercise pass, then commit it (`feat(l01): solve e04 encoder_delta`).
2. E08 and E11 have constraints written in their doc comments. Respect them, because the constraint is the lesson.
3. When everything is green, **delete the `#![allow(unused_variables)]` line** at the top of `lib.rs`, then run `cargo clippy --workspace --all-targets -- -D warnings`. Fix whatever clippy says, and make sure you understand *why*.

---

## 9. ✅ Concept check

Answer the six L01 questions in [`LEARNING_LOG.md`](../../LEARNING_LOG.md) **in your own words** before opening the PR. I'll review those answers along with your code.

## 10. Coming next: L02 Ownership

This is the lesson that makes Rust *Rust*. Preview question to think about: in Python, `b = a` for a list makes two names for one list. In C++, `b = a` for a `std::vector` deep-copies. **In Rust, `let b = a;` for a `Vec` does neither.** What do you think happens to `a`?
