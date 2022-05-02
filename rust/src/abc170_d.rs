use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }
    let max = *an.iter().max().unwrap();

    an.push(0);
    an.push(1010101);
    an.sort();

    let mut used = vec![false; max + 1];
    let mut ans = 0;
    for i in 1..=n {
        let a = an[i];
        if used[a] == false && an[i] != an[i - 1] && an[i] != an[i + 1] {
            used[a] = true;
            ans += 1;
        }
        let mut sum = a;
        while sum <= max {
            used[sum] = true;
            sum += a;
        }
    }
    println!("{}", ans);
}
