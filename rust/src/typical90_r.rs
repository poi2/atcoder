#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn round_wheel_pos(hight: f64, round_time: f64, time: f64) -> (f64, f64) {
    let rad = time / round_time * 2.0 * std::f64::consts::PI;
    return (hight * rad.sin() * -1.0, hight - hight * rad.cos());
}

// NOTE x.atan2(y) => radian
//      角度にする場合は radian * 180 / PI => 角度
fn rad(x: f64, y: f64, z: f64) -> f64 {
    let xy = (x.powf(2.0) + y.powf(2.0)).sqrt();
    z.atan2(xy) * 180.0 / std::f64::consts::PI
}

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        eq: [f64; q],
    };

    for e in eq {
        let (wy, wz) = round_wheel_pos(l / 2.0, t, e);

        let ans = rad(x, y - wy, wz);
        println!("{}", ans);
    }
}
