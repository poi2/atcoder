use proconio::input;
use fenwicktree::Sum;

// FenwickTree の xor 版を作成する必要がある
// 現時点では他の人のコードを読んで写経するくらいしかできなかった
fn main() {
    input! {
        n: usize, q: usize,
        an: [isize; n],
        queries: [(usize, i64, i64); q],
    }

    let mut tree = fenwicktree::FenwickTree::new(n * 10);
    for (idx, &a) in an.iter().enumerate() {
        tree.add(idx as i64 + 1, a);
    }

    for (t, x, y) in queries {
        match t {
            1 => tree.add(x, y as isize),
            _ => println!("{}", tree.sum((x, y))),
        }
    }
}

mod fenwicktree {
    use std::clone::Clone;
    use std::convert::From;
    use std::ops::{Add, AddAssign, Sub, BitXor, BitXorAssign};

    // binaryIndexTree
    #[derive(Clone, Debug)]
    pub struct FenwickTree<T> {
        array: Vec<T>,
    }

    impl<T> FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign + BitXorAssign,
    {
        pub fn new(size: usize) -> FenwickTree<T> {
            let v: Vec<T> = vec![T::from(0u8); size+1];
            Self { array: v }
        }

        pub fn add(&mut self, mut i: i64, x: T) {
            let n = self.array.len();
            while i as usize <= n {
                self.array[i as usize] ^= x;
                i += i & i.wrapping_neg();
            }
        }
    }

    pub trait Sum<T, U> {
        fn sum(&self, i: T) -> U;
    }

    impl<T> Sum<i64, T> for FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign + BitXorAssign,
    {
        fn sum(&self, mut i: i64) -> T {
            if i==0 {
                return T::from(0u8);
            }
            let mut s = T::from(0u8);

            while i > 0 {
                s ^= self.array[i as usize];
                i -= i & i.wrapping_neg();
            }
            s
        }
    }

    impl<T> Sum<(i64, i64), T> for FenwickTree<T>
    where
        T: Add + Sub + Clone + Copy + From<u8> + AddAssign + BitXorAssign,
        T: Sub<Output = T> + BitXor<Output = T>,
    {
        fn sum(&self, i: (i64, i64)) -> T {
            let sum_l = <FenwickTree<T> as Sum<i64, T>>::sum(self, i.0 - 1);
            let sum_r = <FenwickTree<T> as Sum<i64, T>>::sum(self, i.1);
            sum_r ^ sum_l
        }
    }
}

