use proconio::{fastout, input};
use std::cmp::min;

// 007 - CP Classes

#[fastout]
fn main() {
    input! {
        n: isize,
        mut a: [isize; n],
        q: isize,
        b: [isize; q],
    }

    a.push(std::isize::MIN);
    a.push(std::isize::MAX);
    a.sort_unstable();
    a.dedup();

    for i in b {
        match a.binary_search(&i) {
            Ok(_) => println!("0"),
            Err(idx) => {
                if idx == 0 {
                    println!("{}", (a[0] - i).abs());
                } else if idx == a.len() - 1 {
                    println!("{}", (a[idx - 1] - i).abs());
                } else {
                    let r = min((a[idx] - i).abs(), (a[idx - 1] - i).abs());
                    println!("{}", r);
                }
            }
        }
    }
}
