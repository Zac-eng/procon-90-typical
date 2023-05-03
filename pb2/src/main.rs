use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    if n % 2 == 1 {}
    else {
        let top: usize = 2usize.pow(n);
        for i in 0..top {
            let mut parenthes_vec: Vec<usize> = Vec::new();
            push_parenthes(&mut parenthes_vec, i, n);
            if judge_valid_parenthes(&parenthes_vec) == 1 {
                for bin in &parenthes_vec {
                    if *bin == 0 {print!("(");}
                    else {print!(")");}
                }
                print!("\n");
            }
            parenthes_vec.clear();
        }
    }
}

fn push_parenthes(parenthes_vec: &mut Vec<usize>, mut decimal: usize, n: u32) {
    for _i in 0..n as usize {
        parenthes_vec.push(decimal % 2);
        decimal /= 2;
    }
    parenthes_vec.reverse();
}

fn judge_valid_parenthes(parenthes: &Vec<usize>) -> usize {
    let mut zero_count: usize = 0;
    let mut one_count: usize = 0;

    for bin in parenthes {
        if *bin == 0 {zero_count += 1;}
        else {
            one_count += 1;
            if one_count > zero_count {return 0}
        }
    }
    if zero_count != one_count {return 0}
    1
}