use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize, m: usize, q: usize,
        mut wvn: [(usize, usize); n],
        xm: [usize; m],
        qs: [(usize, usize); q],
    }

    wvn.sort_by_key(|&(_w, v)| v);
    wvn.reverse();

    for (l, r) in qs {
        let mut active = xm.clone();
        active.drain((l - 1)..r);
        active.sort();

        let (ans, _len) = wvn
            .iter()
            .fold((0, active.len()), |(acc, len), &(w, v)| {
                let id = active.lower_bound(&w);
                if id == len { return (acc, len); }

                active.remove(id);
                return (acc + v, len - 1);
            });
        println!("{}", ans);
    }
}
