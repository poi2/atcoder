use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = std::usize::MAX;
    let lim = 10_usize.pow(6);
    let mut i = 0;
    let mut j = lim;

    while i <= lim.min(j) {
        loop {
            let memo = f(i, j);
            if memo < n { break; }
            ans = ans.min(f(i, j));

            if j == 0 { break; }
            j -= 1;
        }

        i += 1;
    }

    println!("{}", ans);
}

fn f(a: usize, b: usize) -> usize {
    a.pow(3) + a.pow(2) * b + a * b.pow(2) + b.pow(3)
}
