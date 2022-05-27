use proconio::{fastout, input};

// 014 - We Used to Sing a Song Together
// https://atcoder.jp/contests/typical90/tasks/typical90_n

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut minimum = 0;
    for i in 0..n {
        minimum += (a[i] - b[i]).abs();
    }

    println!("{}", minimum);
}
