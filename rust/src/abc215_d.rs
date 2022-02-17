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


// NOTE エラトステネスの篩
pub struct Sieve {
	sieve: Vec<u64>,
	n: usize,
}
impl Sieve {
	const BIT_LENGTH: usize = std::mem::size_of::<u64>() * 8;

	pub fn new(n: usize) -> Self {
		let base = (1..Self::BIT_LENGTH).step_by(2).fold(0, |r, i| r | 1 << i);
		let mut sieve = vec![base; n / Self::BIT_LENGTH + 1];
		sieve[0] ^= 0b0110;
		for p in (3..).step_by(2).take_while(|&i| i * i < n) {
			if sieve[p / Self::BIT_LENGTH] & (1 << p % Self::BIT_LENGTH) != 0 {
				for q in (p * p..=n).step_by(2 * p) {
					sieve[q / Self::BIT_LENGTH] &= std::u64::MAX ^ (1 << q % Self::BIT_LENGTH);
				}
			}
		}
		let r = n % Self::BIT_LENGTH;
		*sieve.last_mut().unwrap() &= ((1 << r) - 1) | 1 << r;
		Self { sieve, n }
	}

	pub fn is_prime<T>(&self, p: T) -> bool
	where
		T: std::convert::TryInto<usize>,
	{
		if let Ok(p) = p.try_into() {
			assert!(p <= self.n);
			self.sieve[p / Self::BIT_LENGTH] & (1 << p % Self::BIT_LENGTH) != 0
		} else {
			false
		}
	}

	pub fn primes<'a, T>(&'a self) -> impl 'a + std::iter::Iterator<Item = T>
	where
		T: std::convert::TryFrom<usize>,
	{
		let mut it = self.sieve.iter();
		let mut bits = 0;
		let mut p = 0;
		std::iter::from_fn(move || {
			while bits == 0 {
				if let Some(next) = it.next().cloned() {
					bits = next;
					p += Self::BIT_LENGTH;
				} else {
					return None;
				}
			}
			let lsb = bits & 0u64.wrapping_sub(bits);
			bits ^= lsb;
			let q = (p - Self::BIT_LENGTH) + lsb.trailing_zeros() as usize;

			T::try_from(q).ok()
		})
	}
}