use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
        c: i32,
    }
    let coin_list: [i32; 3] = sort_coins(a, b, c);
    let mut min_coins = 9999;
    for i in 0..=n / coin_list[2] {
        for j in 0..=(n - coin_list[2] * i) / coin_list[1] {
            if (n - coin_list[2] * i - coin_list[1] * j) % coin_list[0] == 0 {
                let k = (n - coin_list[2] * i - coin_list[1] * j) / coin_list[0];
                let total_coins = i + j + k;
                min_coins = min_coins.min(total_coins);
            }
        }
    }
    println!("{}", min_coins);
}

fn sort_coins(a: i32, b: i32, c: i32) -> [i32; 3] {
    let mut coins = [a, b, c];
    coins.sort();
    return coins;
}
