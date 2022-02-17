#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, m: usize,
        an: [usize; n],
    }
    let mut primes: BTreeSet<usize> = BTreeSet::new();
    an.iter().for_each(|&a| {
        for (prime, _count) in prime_factorization(a) {
            primes.insert(prime);
        }
    });

    let mut list = vec![true; m + 1];
    list[0] = false;
    primes.iter().for_each(|&p| {
        let mut i = p;
        while i <= m {
            list[i] = false;
            i += p;
        }
    });

    let ans_list = list.iter().enumerate()
        .filter(|&(_i, &bool)| bool )
        .map(|(i, _bool)| i)
        .collect::<Vec<usize>>();

    println!("{}", ans_list.len());
    ans_list.iter().for_each(|&ans| println!("{}", ans) );
}

// NOTE 素因数分解
fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    fn factor_sub(n: usize, m: usize) -> (usize, usize) {
        let mut c = 0;
        let mut x = n;
        while x % m == 0 {
            c += 1;
            x /= m;
        }
        return (c, x);
    }

    let mut ans = vec![];
    let (c, n) = factor_sub(n, 2);
    if c > 0 { ans.push((2, c)); }

    let mut x = 3;
    let mut m = n;
    while x * x <= m {
        let (c, n) = factor_sub(m, x);
        if c > 0 { ans.push((x, c)); }
        m = n;
        x += 2;
    }
    if m > 1 { ans.push((m, 1)); }

    return ans;
}
