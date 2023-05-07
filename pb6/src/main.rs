use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }
    let given_chars: Vec<char> = s.chars().collect();
    let mut min_chars: Vec<char> = Vec::new();
    choose_min(0, n, k, &given_chars, &mut min_chars);
    print_chars(min_chars);
}

fn choose_min(start:usize, str_len: usize, choose_num: usize, given_chars: &Vec<char>, min_chars: &mut Vec<char>) {
    if choose_num ==  0 {return}
    let possible_range = str_len - choose_num + 1;
    let mut min_pair: (usize, char) = (start, given_chars[start]);
    for index in start..possible_range {
        if given_chars[index] < min_pair.1 {
            min_pair.0 = index;
            min_pair.1 = given_chars[index];
        }
    }
    min_chars.push(min_pair.1);
    choose_min(min_pair.0 + 1, str_len, choose_num - 1, given_chars, min_chars);
    return
}

fn print_chars(chars: Vec<char>) {
    for charactor in chars {
        print!("{}", charactor);
    }
    print!("\n");
}