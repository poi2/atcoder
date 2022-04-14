use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: (usize, usize),
        b: (usize, usize),
        ss: [String; n],
    }
    let mut map = vec![];
    map.push(vec![false; n + 2]);
    for s in ss {
        let mut line = vec![];
        line.push(false);
        s.chars()
            .map(|c| c == '.' )
            .for_each(|b| if b { line.push(true) } else { line.push(false) } );
        line.push(false);
        map.push(line);
    }
    map.push(vec![false; n + 2]);

    let d = a.0.max(b.0) - a.0.min(b.0) + a.1.max(b.1) - a.1.min(b.1);
    if d.is_odd() {
        println!("-1");
        std::process::exit(0);
    }

    if map[a.0][a.1] == false || map[b.0][b.1] == false {
        println!("-1");
        std::process::exit(0);
    }

    let dirs: Vec<(isize, isize)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
    const INF: u128 = std::u128::MAX;
    let mut q1 = vec![];
    let mut q2 = vec![];
    let mut dist = vec![vec![INF; n + 2]; n + 2];
    let mut depth = 1;

    q1.push(a);
    dist[a.0][a.1] = 0;
    while let Some((x, y)) = q1.pop() {
        for &(dx, dy) in dirs.iter() {
            for i in 1.. {
                let nx = (x as isize + dx * i) as usize;
                let ny = (y as isize + dy * i) as usize;
                if !map[nx][ny] || dist[nx][ny] < depth { break; }
                if dist[nx][ny] > depth { q2.push((nx, ny)); }
                dist[nx][ny] = depth;
            }
        }

        if q1.is_empty() && !q2.is_empty() {
            q1 = q2;
            q2 = vec![];
            depth += 1;
        }
    }

    println!("{}",
        match dist[b.0][b.1] {
            INF => -1,
            num => num as isize,
        }
    );
}

// fn main2() {
//     input! {
//         n: usize,
//         a: (usize, usize),
//         b: (usize, usize),
//         ss: [String; n],
//     }
//     let mut map = vec![];
//     map.push(vec![false; n + 2]);
//     for s in ss {
//         let mut line = vec![];
//         line.push(false);
//         s
//                 .chars()
//                 .map(|c| c == '.' )
//                 .for_each(|b| if b { line.push(true) } else { line.push(false) } );
//         line.push(false);
//         map.push(line);
//     }
//     map.push(vec![false; n + 2]);

//     if a.0 == b.0 && a.1 == b.1 {
//         println!("0");
//         std::process::exit(0);
//     }

//     let d = a.0.max(b.0) - a.0.min(b.0) + a.1.max(b.1) - a.1.min(b.1);
//     if d.is_odd() || map[a.0][a.1] == false || map[b.0][b.1] == false {
//         println!("-1");
//         std::process::exit(0);
//     }

//     let dirs: Vec<(isize, isize)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
//     const INF: u128 = std::u128::MAX;
//     let mut dist = vec![vec![vec![INF; 4]; n + 2]; n + 2];
//     let mut queue = VecDeque::new();

//     for i in 0..4 {
//         // 本来 depth 0 ではあるが、簡略化するため 1 を入れておく
//         queue.push_back((1, i, a.0, a.1));
//     }

//     while let Some((depth, z, x, y)) = queue.pop_front() {
//         if dist[x][y][z] <= depth { continue; }
//         dist[x][y][z] = depth;

//         for (dz, &(dx, dy)) in dirs.iter().enumerate() {
//             let nx = (x as isize + dx) as usize;
//             let ny = (y as isize + dy) as usize;
//             if !map[nx][ny] { continue; }
//             if z == dz {
//                 queue.push_front((depth, dz, nx, ny));
//             } else {
//                 queue.push_back((depth + 1, dz, nx, ny));
//             }
//         }
//     }

//     let ans = match *dist[b.0][b.1].iter().min().unwrap() {
//         INF => -1,
//         num => num as isize,
//     };
//     println!("{}", ans)
// }
