use proconio::input;

fn main() {
    input! {
        n: usize,
        score: [(usize, usize);n],
        q: usize,
        ranges: [(usize, usize);q],
    }
    let mut cumulative_sum_list: Vec<(usize, usize)> = vec![(0, 0)];
    culc_cum_sum(&score, &mut cumulative_sum_list);
    print_sum(ranges, cumulative_sum_list);
}

fn culc_cum_sum(score: &Vec<(usize, usize)>, cumulative_sum_list: &mut Vec<(usize, usize)>) {
    let mut class_one_sum: usize = 0;
    let mut class_two_sum: usize = 0;
    for personal_score in score {
        if personal_score.0 == 1 {class_one_sum += personal_score.1;}
        else if personal_score.0 == 2 {class_two_sum += personal_score.1;}
        cumulative_sum_list.push((class_one_sum, class_two_sum));
    }
}

fn print_sum(ranges: Vec<(usize, usize)>, cumulative_sum_list: Vec<(usize, usize)>) {
    for range in ranges {
        let class_one_sum: usize = cumulative_sum_list[range.1].0 - cumulative_sum_list[range.0 - 1].0;
        let class_two_sum: usize = cumulative_sum_list[range.1].1 - cumulative_sum_list[range.0 - 1].1;
        println!("{} {}", class_one_sum, class_two_sum);
    }
}
