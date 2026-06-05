# ternary-fib

**Ternary Fibonacci, Tribonacci, and balanced ternary arithmetic.**

The Fibonacci sequence is 1, 1, 2, 3, 5, 8, 13… — each number is the sum of the two before it. But what happens when addition wraps? When the result is always in `{-1, 0, +1}`?

It turns out ternary Fibonacci has **period 8**: `1, 1, -1, 0, -1, -1, 1, 0`. The sequence loops forever through a specific pattern. Ternary Tribonacci has **period 13**. These are the Pisano periods for modulus 3 — and they're the fundamental rhythms of cyclic ternary systems.

This crate computes these sequences, finds their periods, and provides the balanced ternary arithmetic that makes it all work.

## What's Inside

- **`ternary_add(a, b)`** — balanced ternary addition: always returns {-1, 0, +1}, wraps mod 3
- **`fibonacci(a, b, n)`** — ternary Fibonacci sequence starting from any two seeds
- **`tribonacci(a, b, c, n)`** — ternary Tribonacci (3-term recurrence)
- **`find_period(seq)`** — find the repeating period of any ternary sequence
- **`pisano_period(modulus)`** — compute the Pisano period for Fibonacci under a given modulus
- **`lucas_ternary(n)`** — ternary Lucas sequence (closely related to Fibonacci)
- **`nega_fibonacci(n)`** — Fibonacci with negative indices, in ternary

## Quick Example

```rust
use ternary_fib::*;

// Ternary Fibonacci: period 8
let seq = fibonacci(1, 1, 20);
// [1, 1, -1, 0, -1, -1, 1, 0, 1, 1, -1, 0, ...] — repeats every 8

let period = find_period(&seq);
assert_eq!(period, 8); // The fundamental ternary rhythm

// Ternary Tribonacci: period 13
let trib = tribonacci(1, 1, 1, 30);
assert_eq!(find_period(&trib), 13); // A longer cycle

// Pisano period for mod 3
assert_eq!(pisano_period(3), 8); // Fibonacci mod 3 repeats every 8

// Custom seeds change the starting point but not the period
let custom = fibonacci(-1, 1, 20);
// Same period 8, different phase
```

## The Insight

**Period 8 is the heartbeat of ternary cyclic dynamics.** In the broader ternary ecosystem, Z₃ cyclic dominance (rock-paper-scissors) creates oscillations with period ~50. But at the arithmetic level — pure addition mod 3 — the rhythm is period 8. This is the *inner* clock, the faster pulse underneath the larger oscillation. Fibonacci timing (period 8) as a natural conversation rhythm is why the ten-forward conversation engine uses 8-tick cycles.

**Use cases:**
- **Algorithmic music** — period-8 as a rhythmic foundation, period-13 as an alternative time signature
- **Sequence analysis** — detect cyclic structure in ternary data
- **Cryptography** — ternary Fibonacci as a simple PRNG with known period
- **Number theory education** — Pisano periods are a beautiful bridge between arithmetic and cyclic dynamics
- **Multi-agent timing** — Fibonacci-based round-robin scheduling

## See Also

- **ternary-collatz** — another famous integer sequence projected to ternary
- **ternary-loop** — general-purpose period detection in ternary signals
- **ternary-phase** — phase relationships between Fibonacci-timed oscillators
- **ternary-polyrhythm** — layer multiple Fibonacci rhythms for polyrhythmic patterns

## Install

```bash
cargo add ternary-fib
```

## License

MIT
