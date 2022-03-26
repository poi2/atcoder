use proconio::input;

fn main() {
    input! {
        h: usize, w: usize, n: usize, m: usize,
        mut abn: [(usize, usize); n],
        cdm: [(usize, usize); m],
    }
    let mut map = vec![vec![false; w]; h];
    for (mut c, mut d) in cdm {
        c -= 1;
        d -= 1;
        map[c][d] = true;
    }
    let mut blocks = map.clone();

    abn.sort_by_key(|(a, _)| *a);
    for (mut a, mut b) in abn.iter() {
        a -= 1;
        b -= 1;
        map[a][b] = true;
        let mut i = 1;
        while b >= i && map[a][b - i] == false {
            map[a][b - i] = true;
            i += 1;
        }
        let mut i = 1;
        while b + i < w && map[a][b + i] == false {
            map[a][b + i] = true;
            i += 1;
        }
    }

    abn.sort_by_key(|(_, b)| *b);
    for (mut a, mut b) in abn.iter() {
        a -= 1;
        b -= 1;
        let mut i = 1;
        while a >= i && blocks[a - i][b] == false {
            blocks[a - i][b] = true;
            map[a - i][b] = true;
            i += 1;
        }
        let mut i = 1;
        while a + i < h && blocks[a + i][b] == false {
            blocks[a + i][b] = true;
            map[a + i][b] = true;
            i += 1;
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] { ans += 1 }
        }
    }

    ans -= m;
    println!("{}", ans);
}
