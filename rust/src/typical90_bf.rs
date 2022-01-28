use proconio::input;
use std::collections::HashMap;

// NOTE 手続き的記述方法
// #[allow(dead_code)]
// fn next_gen(mut n: usize) -> usize {
//     let mut nxt = n;
//     while n != 0 {
//         nxt += n % 10;
//         n /= 10;
//     }
//     return nxt % 100000;
// }
// fn main() {
//     input! { mut n: usize, k: usize }
//     let mut used = HashMap::new();
//     let mut values = vec![];
//     let mut ans_idx = k - 1;
//     for i in 0..k {
//         n = next_gen(n);
//         if used.contains_key(&n) {
//             let loop_start = used[&n];
//             let loop_size = i - loop_start;
//             ans_idx = (k - loop_start - 1) % loop_size + loop_start;
//             break;
//         } else {
//             used.insert(n, i);
//             values.push(n);
//         }
//     }
//     println!("{}", values[ans_idx]);
// }

// NOTE 構造体を使ってメモ化の実装を楽にする
fn main() {
    input! { mut n: usize, k: usize }
    
    let mut my = My::new(k, n);
    let mut i = 0;
    while i < k && !my.solve(i) { i += 1; }

    println!("{}", my.ans());
}

struct My {
    k: usize,
    n: usize,
    used: HashMap<usize, usize>,
    values: Vec<usize>,
    ans_idx: usize,
}
impl My {
    pub fn new(k: usize, n: usize) -> Self {
        Self { k, n, used: HashMap::new(), values: vec![], ans_idx: k - 1 }
    }

    fn ans(self) -> usize {
        return self.values[self.ans_idx];
    }

    fn solve(&mut self, i: usize) -> bool {
        self.update_n();
        if self.used.contains_key(&self.n) {
            let loop_start = self.used[&self.n];
            let loop_size = i - loop_start;
            self.ans_idx = (self.k - loop_start - 1) % loop_size + loop_start;
            return true
        } else {
            self.used.insert(self.n, i);
            self.values.push(self.n);
            return false;
        }
    }

    fn update_n(&mut self) {
        let mut tmp = self.n;
        let mut nxt = self.n;
        while tmp != 0 {
            nxt += tmp % 10;
            tmp /= 10;
        }
        self.n = nxt % 100000;
    }
}
