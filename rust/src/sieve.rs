use std::iter::empty;

fn sieve(limit: usize) -> Box<dyn Iterator<Item = usize>> {
    if limit < 2 { return Box::new(empty()) }

    let mut is_prime = vec![true; limit+1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false }
    let sqrtlmt = (limit as f64).sqrt() as usize + 1;

    for num in 2..sqrtlmt {
        if is_prime[num] {
            let mut multiple = num * num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }

    Box::new(
        is_prime
            .into_iter()
            .enumerate()
            .filter_map(|(p, is_prm)| if is_prm { Some(p) } else { None })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(sieve(10).collect::<Vec<usize>>(), vec![2, 3, 5, 7]);
    }
}
