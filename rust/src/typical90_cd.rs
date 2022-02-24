#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn digit_size(mut num: u128) -> u128 {
    let mut digit = 0;
    while num > 0 {
        digit += 1;
        num /= 10;
    }
    return digit;
}

fn sum_arithmetic_progressions(a0: u128, an: u128, d: u128) -> u128 {
    let n = (an - a0) / d + 1;
    return (a0 + an) * n / 2;
}

fn main() {
    input! {
        l: u128, r: u128,
    }
    println!("{}", solve(l, r));
}

fn solve(l: u128, r: u128) -> u128 {
    let mod_num = 10_u128.pow(9) + 7;
    let mut ans = 0;
    for digit in digit_size(l)..=digit_size(r) {
        let min = 10_u128.pow((digit - 1) as u32);
        let max = min * 10 - 1;

        let a0 = if min < l { l } else { min };
        let an = if r < max { r } else { max };

        ans += sum_arithmetic_progressions(a0 * digit, an * digit, digit);
        ans %= mod_num;
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_arithmetic_progressions() {
        assert_eq!(sum_arithmetic_progressions(3, 5, 1), 12);
        assert_eq!(sum_arithmetic_progressions(20, 22, 2), 42);
        assert_eq!(sum_arithmetic_progressions(2 * 98, 2 * 99, 2), 394);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(98, 100), 694);
        assert_eq!(solve(1001,869120), 59367733);
        assert_eq!(solve(381453331666495446, 746254773042091083), 584127830);
    }
}
