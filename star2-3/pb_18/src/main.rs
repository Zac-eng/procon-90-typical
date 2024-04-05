use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64;q],
    }
    let radius = l / 2.;
    for time in e {
        //the coordinate of the Ferris wheel
        let fw_y: f64 = -1. * radius  * (2. * PI * time / t ).sin();
        let fw_z: f64 = radius * (1.0 - (2.0 * PI * time / t).cos());
        
        let vec_xy: [f64;2] = [x, y - fw_y];
        let xy_abs = (vec_xy[0].powi(2) + vec_xy[1].powi(2)).sqrt();
        let atan: f64 = fw_z.atan2(xy_abs);
        println!("{}", atan * 360. / 2. / PI);
    }
}
