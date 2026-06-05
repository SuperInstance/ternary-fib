#![forbid(unsafe_code)]

/// Balanced ternary addition: result is always -1, 0, or +1
/// (wraps mod 3 mapped to ternary digits)
pub fn ternary_add(a: i8, b: i8) -> i8 {
    let sum = a + b;
    match sum {
        -2 => 1,
        -1 => -1,
        0 => 0,
        1 => 1,
        2 => -1,
        _ => 0,
    }
}

/// Ternary Fibonacci sequence: each term is ternary_add of previous two
pub fn fibonacci(a: i8, b: i8, n: usize) -> Vec<i8> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![a]; }
    let mut seq = vec![a, b];
    for _ in 2..n {
        let len = seq.len();
        seq.push(ternary_add(seq[len - 2], seq[len - 1]));
    }
    seq
}

/// Ternary Tribonacci sequence: each term is ternary_add of previous three
pub fn tribonacci(a: i8, b: i8, c: i8, n: usize) -> Vec<i8> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![a]; }
    if n == 2 { return vec![a, b]; }
    let mut seq = vec![a, b, c];
    for _ in 3..n {
        let len = seq.len();
        seq.push(ternary_add(ternary_add(seq[len - 3], seq[len - 2]), seq[len - 1]));
    }
    seq
}

/// Find the period of a ternary sequence
pub fn find_period(seq: &[i8]) -> usize {
    if seq.is_empty() { return 0; }
    for p in 1..=seq.len() / 2 {
        let mut is_period = true;
        for i in p..seq.len() {
            if seq[i] != seq[i - p] {
                is_period = false;
                break;
            }
        }
        if is_period { return p; }
    }
    seq.len()
}

/// Compute the Pisano period for a given modulus
pub fn pisano_period(modulus: usize) -> usize {
    if modulus <= 1 { return 1; }
    let mut prev = 0usize;
    let mut curr = 1usize;
    for i in 1..=modulus * modulus * 6 {
        let next = (prev + curr) % modulus;
        prev = curr;
        curr = next;
        if prev == 0 && curr == 1 {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_add_basic() {
        assert_eq!(ternary_add(1, 1), -1);
        assert_eq!(ternary_add(-1, -1), 1);
        assert_eq!(ternary_add(1, -1), 0);
        assert_eq!(ternary_add(0, 0), 0);
    }

    #[test]
    fn test_ternary_add_identity() {
        assert_eq!(ternary_add(0, 1), 1);
        assert_eq!(ternary_add(0, -1), -1);
        assert_eq!(ternary_add(1, 0), 1);
        assert_eq!(ternary_add(-1, 0), -1);
    }

    #[test]
    fn test_fibonacci_empty() {
        assert!(fibonacci(1, -1, 0).is_empty());
    }

    #[test]
    fn test_fibonacci_one() {
        assert_eq!(fibonacci(1, -1, 1), vec![1]);
    }

    #[test]
    fn test_ternary_fibonacci_period_8() {
        // Start with 1, 1 and generate enough terms
        let seq = fibonacci(1, 1, 32);
        // The period should be 8 (Pisano period for mod 3)
        let p = find_period(&seq);
        assert_eq!(p, 8);
    }

    #[test]
    fn test_fibonacci_values() {
        let seq = fibonacci(1, 1, 10);
        // 1, 1, -1, 0, -1, -1, 1, 0, 1, 1
        assert_eq!(seq, vec![1, 1, -1, 0, -1, -1, 1, 0, 1, 1]);
    }

    #[test]
    fn test_tribonacci_basic() {
        let seq = tribonacci(1, 1, 1, 8);
        assert_eq!(seq.len(), 8);
    }

    #[test]
    fn test_tribonacci_zero_start() {
        let seq = tribonacci(0, 0, 1, 6);
        assert_eq!(seq[0], 0);
        assert_eq!(seq[1], 0);
        assert_eq!(seq[2], 1);
    }

    #[test]
    fn test_find_period_constant() {
        assert_eq!(find_period(&[1, 1, 1, 1]), 1);
    }

    #[test]
    fn test_find_period_alternating() {
        assert_eq!(find_period(&[1, -1, 1, -1]), 2);
    }

    #[test]
    fn test_pisano_period_3() {
        assert_eq!(pisano_period(3), 8);
    }

    #[test]
    fn test_pisano_period_1() {
        assert_eq!(pisano_period(1), 1);
    }

    #[test]
    fn test_pisano_period_2() {
        assert_eq!(pisano_period(2), 3);
    }

    #[test]
    fn test_fibonacci_repeats_after_period() {
        let seq = fibonacci(1, 1, 16);
        // Period is 8, so seq[0..8] == seq[8..16]
        assert_eq!(seq[0..8], seq[8..16]);
    }
}
