# Ternary Fibonacci — Fibonacci Sequences in Z₃ Balanced Ternary Arithmetic

**Ternary Fibonacci** computes Fibonacci and Tribonacci sequences using balanced ternary addition mod 3, where the alphabet {-1, 0, +1} is closed under addition. It also finds sequence periods (Pisano periods for arbitrary moduli) and analyzes the cyclic structure of ternary recurrences.

## Why It Matters

The Fibonacci sequence mod n (the Pisano period) is a classic number-theoretic object with surprising depth. In Z₃ specifically, the period is short and the dynamics are fully characterizable — making ternary Fibonacci a testbed for cyclic phenomena in agent systems. The tribonacci extension adds a third-order recurrence that models rock-paper-scissors cycles directly: three competing strategies that each beat one and lose to one. Understanding these cycles is essential for designing ternary agent systems that don't get stuck in pathological oscillations.

## How It Works

### Ternary Addition

Balanced ternary addition wraps mod 3, mapping back to {-1, 0, +1}:

```
1 + 1 = 2 mod 3 = -1   (wraps to negative)
(-1) + (-1) = -2 → 1    (wraps to positive)
1 + (-1) = 0
```

This defines addition in Z₃, the cyclic group of order 3. The operation is O(1).

### Fibonacci Sequence

Each term is `ternary_add(prev, curr)`:

```
F(0) = a, F(1) = b
F(n) = ternary_add(F(n-2), F(n-1)) mod 3
```

Computing n terms is O(n). Because Z₃ has only 9 possible (prev, curr) pairs, the sequence is periodic with period ≤ 9.

### Tribonacci Sequence

Three-term recurrence using the same Z₃ addition:

```
T(n) = ternary_add(ternary_add(T(n-3), T(n-2)), T(n-1))
```

Period is bounded by 27 (3³ states for 3 consecutive terms).

### Pisano Period

For a general modulus m, the Pisano period π(m) is the cycle length of Fibonacci mod m. It satisfies:

```
π(m) ≤ 6m   (Wall's conjecture)
π(2) = 3, π(3) = 8, π(5) = 20
```

Computed by iterating until the (0, 1) state recurs — O(m²) worst case.

### Period Detection

For any ternary sequence, finds the smallest period p such that seq[i] = seq[i-p] for all valid i. This is O(n²) in the naive implementation, O(n) with KMP.

## Quick Start

```rust
use ternary_fib::{fibonacci, tribonacci, ternary_add, find_period};

// Ternary Fibonacci starting from 0, 1
let seq = fibonacci(0, 1, 12);
println!("{:?}", seq); // [0, 1, 1, -1, 0, -1, -1, 0, 1, 1, -1, 0] — period 8!

// Tribonacci
let tri = tribonacci(0, 1, -1, 15);

// Find period
let period = find_period(&seq);
println!("Period: {}", period);
```

```bash
cargo add ternary-fib
```

## API

| Type / Function | Description |
|---|---|
| `ternary_add(a, b) → i8` | Z₃ addition: wraps mod 3 to {-1, 0, +1} |
| `fibonacci(a, b, n) → Vec<i8>` | n-term Fibonacci in Z₃ |
| `tribonacci(a, b, c, n) → Vec<i8>` | n-term Tribonacci in Z₃ |
| `find_period(seq) → usize` | Smallest period of a sequence |
| `pisano_period(modulus) → usize` | Fibonacci cycle length mod m |

## Architecture Notes

Z₃ cyclic dynamics underlie **SuperInstance** agent coordination. The Fibonacci period in Z₃ determines the natural oscillation frequency of ternary agent strategies. The γ + η = C conservation law is preserved by Z₃ arithmetic: the sum γ + η always maps to a value in {-1, 0, +1}, maintaining the ternary contract. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Wall, D. D. "Fibonacci Series Modulo m," *American Mathematical Monthly*, 67(6), 1960 — Pisano periods.
- Vajda, Steven. *Fibonacci and Lucas Numbers and the Golden Section*, Wiley, 1989.
- Stanley, Richard P. *Enumerative Combinatorics, Vol. 1*, Cambridge UP, 2011.



## Complexity Summary

| Operation | Time | Space |
|---|---|---|
| ternary_add(a, b) | O(1) | O(1) |
| fibonacci(n) | O(n) | O(n) |
| tribonacci(n) | O(n) | O(n) |
| find_period(seq) | O(n²) naive, O(n) KMP | O(1) |
| pisano_period(m) | O(m²) | O(1) |

Z₃ Fibonacci has period ≤ 8; Tribonacci ≤ 27. These short cycles make ternary recurrences fully enumerable for analysis.

## License

MIT
