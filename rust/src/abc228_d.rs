use proconio::input;

fn main() {
    // let n = 1048576;
    // input! {
    //     q: usize,
    // }
    // let mut list: Vec<i128> = vec![-1; n + 1];
    // let mut used: Vec<usize> = vec![];

    // for _ in 1..=q {
    //     input! {
    //         t: usize,
    //         x: usize
    //     }
    //     if t == 1 {
    //         let mut h = x;
    //         let mut amari = h % n;
    //         match used.binary_search_by(|prob| prob.cmp(&amari).reverse() ) {
    //             Ok(ans) => {
    //                 println!("ok");
    //                 list[ans] = x;
    //                 used.insert(ans, ans)
    //             },
    //             Err(ans) => {
    //                 println!("err, {}", ans);
    //                 list[ans] = x;
    //                 used.insert(ans, ans)
    //             },
    //         }
    //         // loop {
    //         //     match map.get(&amari) {
    //         //         Some(_ans) => {
    //         //             h += 1;
    //         //             amari = h % n;
    //         //         },
    //         //         None => {
    //         //             map.insert(amari, x);
    //         //             break;
    //         //         }
    //         //     }
    //         // }
    //     } else {
    //         let amari = x % n;
    //         match map.get(&amari) {
    //             Some(ans) => println!("{}", ans),
    //             None => println!("-1"),
    //         }
    //     }
    // }
}
