use proconio::input;
use std::f64::consts::PI;
use nalgebra::Vector3;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64;q],
    }
    for time in e {
        //the coordinate of the Ferris wheel
        let fw_y: f64 = -1. * l  * (2. * PI * time / t ).sin();
        let fw_z: f64 = l * (1.0 - (2.0 * PI * time / t).cos());
        //the vector describes Ferris wheel's position
        let fw_vec = Vector3::new(0., fw_y, fw_z);
        //the vector from Ferris wheel to Takahashi statue
        let fw_t_vec = Vector3::new(x , y , 0.) - fw_vec;
        let fw_t_abs = fw_t_vec.norm();
        let sin = fw_z / fw_t_abs; 
        // let ip = fw_t_vec.dot(&fw_t_x);
        // let fw_t_x_abs = fw_t_x.norm();
        // let cos = ip / fw_t_abs / fw_t_x_abs;
        // println!("{}", cos.acos() * 360. / 2. / PI);
        println!("{}", sin.asin() * 360. / 2. / PI);
    }
}
