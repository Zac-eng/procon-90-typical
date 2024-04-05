use proconio::input;

fn main() {
    input! {
        a: u64,
        b: i32,
        c: u32,
    }
    judge_bigger(a, b, c);
}

fn judge_bigger(a: u64, b: i32, c: u32)
{
    let mut powed: u64 = c as u64;
    let mut b_local = b - 1;
    while b_local > 0 && a >= powed as u64 {
        if a / (c as u64) < powed {return println!("Yes")}
        powed *= c as u64;
        b_local -= 1;
    }
    if a < powed { return println!("Yes")}
    println!("No");
}
