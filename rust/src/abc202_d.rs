#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        mut a: usize, mut b: usize, mut k: u128,
    }
    let mut ans = vec![];
    let n = a + b;
    let mut ncr = NCR::new(n, a);

    for _ in 0..n {
        if a == 0 {
            ans.push("b");
            b -= 1;
        } else if b == 0 {
            ans.push("a");
            a -= 1;
        } else {
            let ncr = ncr.call(a + b - 1, a - 1);
            if k <= ncr {
                ans.push("a");
                a -= 1;
            } else {
                ans.push("b");
                b -= 1;
                k -= ncr;
            }
        }
    }
    println!("{}", ans.join(""));
}

// 組み合わせの数
// 二項定理とパスカルの三角形を利用した計算
// fn binom_pascal(n: usize, k: usize ) -> usize {
//     if k == 0 || k == n { return 1; }

//     let mut p = vec![ 1 ];

//     for _ in 0..n-1 {
//         let mut c = vec![ 1 ];

//         for x in p.windows(2) {
//             c.push( x[0] + x[1] );
//         }
//         c.push( 1 );

//         p = c;
//     }

//     p[k-1] + p[k]
// }


// 組み合わせの数
// パスカルの法則を利用した計算
// nCr = n-1Cr + n-1Cr-1
// if r == 0 => 1 : ひとつも選ばない組み合わせは 1 パターン
// if n == r => 1 : すべてを選ぶ組み合わせは 1 パターン
struct NCR {
    memo: Vec<Vec<Option<u128>>>,
}
impl NCR {
    pub fn new(max_n: usize, max_r: usize) -> Self {
        return Self { memo: vec![vec![None; max_r]; max_n] };
    }

    pub fn call(&mut self, n: usize, r: usize) -> u128 {
        if r == 0 || n == r { return 1; }
        if let Some(ans) = self.memo[n][r] { return ans; }

        let ans = self.call(n - 1, r - 1) + self.call(n - 1, r);
        self.memo[n][r] = Some(ans);
        return ans;
    }
}
