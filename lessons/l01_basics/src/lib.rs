//! # L01 · Basics: exercises
//!
//! Replace every `todo!()` until `cargo test -p l01_basics` is green.
//! Notes: `lessons/l01_basics/README.md`. Don't edit the tests at the bottom.

// Silences "unused variable" warnings while bodies are still `todo!()`.
// 👉 DELETE THIS LINE when all tests pass, then run clippy.
#![allow(unused_variables)]

// ───────────────────────────── Functions & expressions ─────────────────────────────

/// E01: Convert Celsius to Fahrenheit: `F = C × 9/5 + 32`.
///
/// Aim for a one-line body with no `return` and no semicolon.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!()
}

/// E02: Linear speed (m/s) of a wheel, given its rotation speed in RPM and its radius
/// in metres. Negative RPM means the wheel spins backwards, so the speed is negative.
///
/// Hint: one revolution travels one circumference (`2πr`), and there are 60 s in a minute.
/// π lives at `std::f64::consts::PI`.
pub fn wheel_linear_speed(rpm: f64, wheel_radius_m: f64) -> f64 {
    todo!()
}

// ───────────────────────────── Types, casting, overflow ─────────────────────────────

/// E03: Convert a controller output to an 8-bit PWM duty value.
/// Anything below 0 becomes 0 and anything above 255 becomes 255.
///
/// ⚠️ `value as u8` alone is WRONG: `300 as u8 == 44`. Clamp first, or use `u8::try_from`.
/// Hint: integers have a `.clamp(min, max)` method.
pub fn to_pwm(value: i32) -> u8 {
    todo!()
}

/// E04: A wheel encoder exposes a `u16` tick counter that WRAPS (… 65534, 65535, 0, 1 …).
/// Given the previous and current readings, return the signed number of ticks moved,
/// assuming the wheel moved less than half a revolution of the counter (|delta| < 32768).
///
/// Examples: `prev=100, now=110 → 10`, `prev=65530, now=4 → 10` (rolled over forwards),
/// `prev=4, now=65530 → -10` (rolled over backwards).
///
/// Hint: `wrapping_sub`, then reinterpret the bits as `i16`. This is one of the rare
/// places where `as` is exactly the right tool. Work out WHY on paper.
pub fn encoder_delta(prev: u16, now: u16) -> i16 {
    todo!()
}

// ───────────────────────────── Control flow ─────────────────────────────

/// E05: Classify a 1-cell Li-ion battery by its voltage in millivolts.
///
/// | range (mV)   | result       |
/// |--------------|--------------|
/// | < 3300       | `"critical"` |
/// | 3300 – 3599  | `"low"`      |
/// | 3600 – 4099  | `"ok"`       |
/// | ≥ 4100       | `"full"`     |
///
/// Use `match` with range patterns. `&'static str` is the type of a string literal:
/// text baked into the binary that lives for the whole program. You'll learn more in L02/L08.
pub fn battery_status(millivolts: u32) -> &'static str {
    todo!()
}

/// E06: Sum of all numbers BELOW `limit` that are multiples of 3 or 5.
/// `sum_of_multiples(10) == 3 + 5 + 6 + 9 == 23`.
pub fn sum_of_multiples(limit: u32) -> u32 {
    todo!()
}

/// E07: Mean of exactly five range readings.
pub fn average(readings: [f64; 5]) -> f64 {
    todo!()
}

/// E08: Return `(index, value)` of the largest reading. If several readings tie for the
/// maximum, return the FIRST one.
///
/// Constraint: write the loop yourself. No `.iter().max()` or other iterator helpers that
/// find the max for you (you'll earn those in L09).
pub fn max_reading(readings: [i32; 6]) -> (usize, i32) {
    todo!()
}

/// E09: A 2-D LiDAR gives 8 range readings (metres). Return the index of the FIRST
/// reading strictly closer than `threshold_m`, or `None` if the path is clear.
///
/// Use an early `return Some(i)` inside the loop and `None` at the end.
pub fn first_obstacle(ranges_m: [f32; 8], threshold_m: f32) -> Option<usize> {
    todo!()
}

/// E10: Number of Collatz steps to reach 1, starting from `n` (n ≥ 1).
/// Rule: if n is even, n = n / 2; otherwise n = 3n + 1. `collatz_steps(1) == 0`.
///
/// Use a `while` loop.
pub fn collatz_steps(n: u64) -> u32 {
    todo!()
}

/// E11: Number of decimal digits in `n`. Note that `0` has one digit.
///
/// Constraint: use `loop { … break value; }`. No string conversion (`to_string().len()`)
/// and no `ilog10`.
pub fn count_digits(n: u64) -> u32 {
    todo!()
}

/// E12 ⭐ (stretch): `n!` as a `u64`, or `None` if it overflows.
/// `20!` fits in a u64 but `21!` doesn't.
///
/// Hint: `checked_mul` returns `Option<u64>`. Inside a loop you can write
/// `acc = acc.checked_mul(i)?;` because `?` returns `None` early from the function
/// (properly introduced in L05). A `match` works too.
pub fn checked_factorial(n: u32) -> Option<u64> {
    todo!()
}

// ═══════════════════════════════ Tests: do not edit ═══════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn e01_celsius_to_fahrenheit() {
        assert!(approx(celsius_to_fahrenheit(0.0), 32.0));
        assert!(approx(celsius_to_fahrenheit(100.0), 212.0));
        assert!(approx(celsius_to_fahrenheit(-40.0), -40.0));
        assert!(approx(celsius_to_fahrenheit(37.0), 98.6));
    }

    #[test]
    fn e02_wheel_linear_speed() {
        // 60 RPM = 1 rev/s; r = 0.5 m → circumference π m → π m/s
        assert!(approx(wheel_linear_speed(60.0, 0.5), std::f64::consts::PI));
        assert!(approx(wheel_linear_speed(0.0, 0.5), 0.0));
        assert!(approx(
            wheel_linear_speed(-60.0, 0.5),
            -std::f64::consts::PI
        ));
        // 120 RPM = 2 rev/s on a 39 mm wheel (r = 0.0195 m) → 2 · 2π · 0.0195 ≈ 0.245 m/s
        assert!(approx(
            wheel_linear_speed(120.0, 0.0195),
            120.0 * 2.0 * std::f64::consts::PI * 0.0195 / 60.0
        ));
    }

    #[test]
    fn e03_to_pwm_in_range() {
        assert_eq!(to_pwm(0), 0);
        assert_eq!(to_pwm(128), 128);
        assert_eq!(to_pwm(255), 255);
    }

    #[test]
    fn e03_to_pwm_clamps() {
        assert_eq!(to_pwm(-10), 0);
        assert_eq!(to_pwm(256), 255);
        assert_eq!(to_pwm(300), 255); // the classic `as u8` bug would give 44
        assert_eq!(to_pwm(i32::MIN), 0);
        assert_eq!(to_pwm(i32::MAX), 255);
    }

    #[test]
    fn e04_encoder_delta_normal() {
        assert_eq!(encoder_delta(100, 110), 10);
        assert_eq!(encoder_delta(110, 100), -10);
        assert_eq!(encoder_delta(500, 500), 0);
        assert_eq!(encoder_delta(0, 32767), 32767);
    }

    #[test]
    fn e04_encoder_delta_rollover() {
        assert_eq!(encoder_delta(65530, 4), 10);
        assert_eq!(encoder_delta(4, 65530), -10);
        assert_eq!(encoder_delta(65535, 0), 1);
        assert_eq!(encoder_delta(0, 65535), -1);
    }

    #[test]
    fn e05_battery_status_boundaries() {
        assert_eq!(battery_status(0), "critical");
        assert_eq!(battery_status(3299), "critical");
        assert_eq!(battery_status(3300), "low");
        assert_eq!(battery_status(3599), "low");
        assert_eq!(battery_status(3600), "ok");
        assert_eq!(battery_status(4099), "ok");
        assert_eq!(battery_status(4100), "full");
        assert_eq!(battery_status(5000), "full");
    }

    #[test]
    fn e06_sum_of_multiples() {
        assert_eq!(sum_of_multiples(0), 0);
        assert_eq!(sum_of_multiples(1), 0);
        assert_eq!(sum_of_multiples(10), 23);
        assert_eq!(sum_of_multiples(16), 60);
        assert_eq!(sum_of_multiples(1000), 233_168);
    }

    #[test]
    fn e07_average() {
        assert!(approx(average([1.0, 2.0, 3.0, 4.0, 5.0]), 3.0));
        assert!(approx(average([0.0; 5]), 0.0));
        assert!(approx(average([-1.0, 1.0, -1.0, 1.0, 0.0]), 0.0));
        assert!(approx(average([0.1, 0.2, 0.3, 0.4, 0.5]), 0.3));
    }

    #[test]
    fn e08_max_reading() {
        assert_eq!(max_reading([1, 2, 3, 4, 5, 6]), (5, 6));
        assert_eq!(max_reading([-5, -3, -9, -4, -10, -7]), (1, -3));
        assert_eq!(max_reading([9, 1, 2, 3, 4, 5]), (0, 9));
    }

    #[test]
    fn e08_max_reading_ties_return_first() {
        assert_eq!(max_reading([3, 9, 2, 9, -1, 0]), (1, 9));
        assert_eq!(max_reading([7, 7, 7, 7, 7, 7]), (0, 7));
    }

    #[test]
    fn e09_first_obstacle() {
        let scan = [2.0, 1.5, 0.4, 0.2, 3.0, 3.0, 3.0, 3.0];
        assert_eq!(first_obstacle(scan, 0.5), Some(2));
        assert_eq!(first_obstacle(scan, 0.1), None);
        assert_eq!(first_obstacle([0.1; 8], 0.5), Some(0));
        assert_eq!(first_obstacle([5.0; 8], 0.5), None);
    }

    #[test]
    fn e09_first_obstacle_is_strict() {
        // exactly at the threshold is NOT an obstacle
        let scan = [0.5, 0.49, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        assert_eq!(first_obstacle(scan, 0.5), Some(1));
    }

    #[test]
    fn e10_collatz_steps() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(3), 7);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn e11_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(7), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(99), 2);
        assert_eq!(count_digits(12_345), 5);
        assert_eq!(count_digits(u64::MAX), 20);
    }

    #[test]
    fn e12_checked_factorial() {
        assert_eq!(checked_factorial(0), Some(1));
        assert_eq!(checked_factorial(1), Some(1));
        assert_eq!(checked_factorial(5), Some(120));
        assert_eq!(checked_factorial(20), Some(2_432_902_008_176_640_000));
        assert_eq!(checked_factorial(21), None);
        assert_eq!(checked_factorial(100), None);
    }
}
