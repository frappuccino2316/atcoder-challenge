use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut c = vec![vec![0_i32; 10]; 10];

    for i in 0..n {
        for j in 0..=9 {
            c[(s[i][j] as usize) - 48][j] += 1;
        }
    }

    let mut m = vec![0; 10];
    for i in 0..=9 {
        for j in 0..=9 {
            m[i] = max(m[i], 10 * ((c[i][j] as i32) - 1) + j as i32);
        }
    }

    println!("{}", m.iter().min().unwrap());
}
