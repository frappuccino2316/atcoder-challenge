use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: i64,
        s: [String; n],
    }

    let mut h = HashSet::new();
    for (i, v) in s.iter().enumerate() {
        if !h.contains(v) {
            println!("{}", i + 1);
            h.insert(v);
        }
    }
}
