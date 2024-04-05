use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        b: [usize; q],
    }
    let mut sorted_diff: Vec<usize> = a;
    sorted_diff.sort();
    for student_level in b {
        let difference: usize = bin_serch_course(&sorted_diff, student_level);
        println!("{}", difference);
    }
}

fn bin_serch_course(sorted_diff: &Vec<usize>, level: usize) -> usize {
    let mut bin_left: usize = 0;
    let mut bin_right: usize = sorted_diff.len() - 1;
    while bin_right - bin_left > 1 {
        let search_index: usize = (bin_left + bin_right) / 2;
        if sorted_diff[search_index] > level {
            bin_right = search_index;
        } else if sorted_diff[search_index] < level {
            bin_left = search_index;
        } else {return 0}
    }
    return diff_min(level, sorted_diff[bin_left], sorted_diff[bin_right])
}

fn diff_min(targ: usize, minimum: usize, maximum: usize) -> usize {
    if targ <= minimum {return minimum - targ}
    else if targ <= maximum {return min(targ - minimum, maximum - targ)}
    else {return targ - maximum}
}