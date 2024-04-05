use proconio::input;

const MOD: usize = 1000000007;
const ATCODER: [char;7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let only_atcorder: Vec<char> = s.chars().filter(|&c| in_atcorder(c) == 1).collect();
    let atcoder_len: usize = only_atcorder.len();
    let mut dp_list: Vec<Vec<usize>> = vec![vec![0;atcoder_len];7];
    let ans = count_atc(&only_atcorder, &mut dp_list, atcoder_len);
    println!("{}", ans);
}

fn in_atcorder(c: char) -> usize {
    if c == 'a' || c == 't' || c == 'c' || c == 'o' || c == 'd' || c == 'e' || c == 'r'
        {return 1}
    else {return 0}
}

fn count_atc(str_list: &Vec<char>, dp_list: &mut Vec<Vec<usize>>, len: usize) -> usize {
    if str_list[0] == 'a' {dp_list[0][0] = 1;}
    for i in 1..len {
        if str_list[i] == 'a' {
            dp_list[0][i] = modulo(dp_list[0][i - 1] + 1);
        } else {
            dp_list[0][i] = modulo(dp_list[0][i - 1]);
        }
    }
    for atc_index in 1..7 {
        for i in atc_index..len {
            if str_list[i] == ATCODER[atc_index] {
                dp_list[atc_index][i] = modulo(dp_list[atc_index - 1][i - 1] + dp_list[atc_index][i - 1]);
            } else {
                dp_list[atc_index][i] = modulo(dp_list[atc_index][i - 1]);
            }
        }
    }
    dp_list[6][len - 1]
}

fn modulo(n: usize) -> usize {
    n % MOD
}