// https://atcoder.jp/contests/abc226/tasks/abc226_d
// メモ
// 実装できたと思ったが、gcd == 0 のパターンが発生してしまう

use proconio::input;

fn mygcd(mut x: u64, mut y: u64) -> u64 {
    while y > 0 {
        let z = x % y;
        x = y;
        y = z;
    }
    x
}

fn main() {
    input! {
        n: usize,
        points: [[i64; 2]; n],
    }

    let mut magics = vec![];
    for i in 0..n {
        for j in 0..n {
            let pi = &points[i];
            let pj = &points[j];

            let dx = pi[0] - pj[0];
            let dy = pi[1] - pj[1];

            let pdx = if dx >= 0 { dx } else { dx * -1 } as u64;
            let pdy = if dy >= 0 { dy } else { dy * -1 } as u64;

            let gcd = mygcd(pdx, pdy) as i64;
            magics.push([dx / gcd, dy / gcd]);
        }
    }
    println!("{:?}", magics);
}
