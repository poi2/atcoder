// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b

use proconio::input;

fn main() {
    input! {
        d: usize,
        store_count: usize,
        order_count: usize,
        mut store_list: [usize; store_count - 1],
        ordre_list: [usize; order_count]
    }
    store_list.sort();
    store_list.insert(0, 0); // 本店追加
    store_list.push(d); // 12時の位置にも本店を追加

    let mut dist = 0;
    for sid in ordre_list {
        let near_idx = &store_list.binary_search(&sid);
        match near_idx {
            Ok(_idx) => {} // 店舗と配送先が一致した場合、距離を足さない
            Err(idx) => {
                // idx は &usize なので、値が欲しい場合 *idx とする
                let diff_left = sid - store_list[idx - 1]; // &size + i32 は　usize + i32 に値に変換される
                let diff_right = store_list[*idx] - sid;
                let diff = if diff_left < diff_right {
                    diff_left
                } else {
                    diff_right
                };
                dist += diff;
            }
        }
    }
    println!("{}", dist);
}
