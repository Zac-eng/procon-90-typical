use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
        mut b: [usize;n]
    }
    a.sort();
    b.sort();
    let mut distance_sum: usize = 0;
    for i in 0..n {
        distance_sum += distance(a[i], b[i]);
    }
    println!("{}", distance_sum);
}

fn distance(lhs: usize, rhs: usize) -> usize {
    if lhs > rhs {return lhs - rhs}
    else {return rhs - lhs}
}