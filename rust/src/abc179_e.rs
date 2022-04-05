use proconio::input;

fn main() {
    input! {
        n: usize, mut x: usize, m: usize,
    }
    let mut sum = 0;
    const MAX: usize = std::usize::MAX;
    let mut used = vec![MAX; m];
    let mut an = vec![];

    let mut i = 0;
    while i < n {
        if used[x] == MAX {
            used[x] = i;
            an.push(x);
            sum += x;

            x = x.pow(2) % m;
        } else {
            break;
        }
        i += 1;
    }

    if i == n {
        println!("{}", sum);
        std::process::exit(0);
    }

    let s = used[x];
    let loop_size = i - s;
    let loop_count = (n - s) / loop_size;
    let rest_size = (n - s) % loop_size;

    let mut sum_pre_loop = 0;
    for j in 0..s {
        sum_pre_loop += an[j];
    }

    let mut sum_loop = 0;
    for j in s..s + loop_size {
        sum_loop += an[j];
    }

    let mut sum_after_loop = 0;
    for j in s..s + rest_size {
        sum_after_loop += an[j];
    }

    let ans = sum_pre_loop + sum_loop * loop_count + sum_after_loop;
    println!("{}", ans);
}
