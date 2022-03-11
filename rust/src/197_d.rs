use proconio::input;

fn main() {
    input! {
        n: usize,
        top: (f64, f64),
        bottom: (f64, f64),
    }
    let pi = std::f64::consts::PI;
    let theta = pi * 2.0 / (n as f64);

    let pivot = get_pivot(top, bottom);
    let xy1 = rotate_from(top, theta, pivot);
    println!("{} {}", xy1.0, xy1.1);
}

fn get_pivot(top: (f64, f64), bottom: (f64, f64)) -> (f64, f64) {
    return ((top.0 + bottom.0) / 2.0, (top.1 + bottom.1) / 2.0);
}

fn sub_by_pivot(xy: (f64, f64), pivot: (f64, f64)) -> (f64, f64) {
    let x_dash = xy.0 - pivot.0;
    let y_dash = xy.1 - pivot.1;
    return (x_dash, y_dash);
}

fn add_by_pivot(xy: (f64, f64), pivot: (f64, f64)) -> (f64, f64) {
    let x_dash = xy.0 + pivot.0;
    let y_dash = xy.1 + pivot.1;
    return (x_dash, y_dash);
}

fn rotate_from_origin(xy: (f64, f64), theta: f64) -> (f64, f64) {
    let rx = xy.0 * theta.cos() - xy.1 * theta.sin();
    let ry = xy.0 * theta.sin() + xy.1 * theta.cos();
    return (rx, ry);
}

fn rotate_from(xy0: (f64, f64), theta: f64, pivot: (f64, f64)) -> (f64, f64) {
    let xy0_dash = sub_by_pivot(xy0, pivot);
    let xy1_dash = rotate_from_origin(xy0_dash, theta);
    return add_by_pivot(xy1_dash, pivot);
}
