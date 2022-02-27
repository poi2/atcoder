#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }
    let mut map = vec![];
    for s in sn {
        let l = s
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 } )
            .collect::<Vec<u8>>();
        map.push(l);
    }
    for i in 0..n {
        for j in 0..n {
            if any_8(&map, i, j, n) {
                println!("{}", "Yes");
                std::process::exit(0);
            }
        }
    }
    println!("{}", "No");
}

fn any_8(map: &Vec<Vec<u8>>, i: usize, j: usize, n: usize) -> bool {
    let mut count = vec![0_u8; 4];
    for x in 0..6 {
        // 0, 235, 270, 315
        if (i < n - 5) && map[i+x][j] == 1 { count[0] += 1; }
        if (i >= 5 && j < n - 5) && map[i-x][j+x] == 1 { count[1] += 1; }
        if (j < n - 5) && map[i][j+x] == 1 { count[2] += 1; }
        if (i < n - 5 && j < n - 5) && map[i+x][j+x] == 1 { count[3] += 1; }
    }
    return count.iter().any(|&c| c >= 4 );
}
