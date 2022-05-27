use proconio::input;
use std::cmp::min;

// 016 - Minimum Coins
// https://atcoder.jp/contests/typical90/tasks/typical90_p

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut minimum = std::i64::MAX;
    let limit = 9999;

    let mut x = 0;
    while x <= limit {
        let mut y = 0;
        while x + y <= limit {
            let remaining = n - (a * x) - (b * y);
            if remaining % c != 0 || remaining < 0 {
                y += 1;
                continue;
            }
            let z = remaining / c;
            minimum = min(minimum, x + y + z);

            y += 1;
        }
        x += 1;
    }

    println!("{}", minimum);
}
