use proconio::input;

fn main() {
    input!{
        n: usize,
        l: usize,
        k: usize,
        a: [usize;n],
    }
    let mut bin_left: usize = 0;
    let mut bin_right: usize = l / (k + 1);
    let a: Vec<usize> = a.to_vec();
    while bin_right - bin_left > 1 {
        if judge_possible((bin_left + bin_right) / 2, &l, k, &a) == 1 {
            bin_left = (bin_left + bin_right) / 2;
        } else {
            bin_right = (bin_left + bin_right) / 2;
        }
    }
    if judge_possible(bin_right, &l, k, &a) == 1 { println!("{}", bin_right); }
    else { println!("{}", bin_left); }
}

fn judge_possible(part_len: usize, l: &usize, k: usize, a: &Vec<usize>) -> usize {
    let mut current_head: &usize = &0;
    let mut cut_count: usize = 0;
    for cut_pos in a {
        if cut_pos - current_head >= part_len {
            cut_count += 1;
            current_head = cut_pos;
        }
        if cut_count >= k { break }
        else {continue;}
    }
    if l - current_head >= part_len {cut_count += 1}
    if cut_count >= k + 1 { return 1 }
    0
}