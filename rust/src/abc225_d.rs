use itertools::Itertools;
use proconio::input;

fn collect_fronts(mut i: usize, trains: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut fronts = Vec::new();
    while trains[i].0 != 1010101 {
        fronts.push(trains[i].0 + 1);
        i = trains[i].0;
    }
    fronts.reverse();
    fronts
}
fn collect_tails(mut i: usize, trains: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut tails = Vec::new();
    while trains[i].1 != 1010101 {
        tails.push(trains[i].1 + 1);
        i = trains[i].1;
    }
    tails
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let max: usize = 1010101;
    // train (front, tail)
    let mut trains: Vec<(usize, usize)> = vec![(max, max); n];
    let mut ans_list = Vec::new();
    for _ in 0..q {
        input! {
            q_num: usize,
        }
        if q_num == 1 {
            input! {
                mut x: usize,
                mut y: usize,
            }
            x -= 1;
            y -= 1;
            trains[x].1 = y;
            trains[y].0 = x;
        } else if q_num == 2 {
            input! {
                mut x: usize,
                mut y: usize,
            }
            x -= 1;
            y -= 1;
            trains[x].1 = max;
            trains[y].0 = max;
        } else if q_num == 3 {
            input! {
                mut x: usize,
            }
            x -= 1;
            let mut ans = Vec::new();
            ans.append(&mut collect_fronts(x, &trains));
            ans.append(&mut vec![x + 1]);
            ans.append(&mut collect_tails(x, &trains));
            ans_list.push(ans);
        };
    }
    for mut ans in ans_list {
        let mut vec = Vec::new();
        vec.append(&mut vec![ans.len()]);
        vec.append(&mut ans);
        println!("{}", vec.into_iter().join(" "));
    }
}
