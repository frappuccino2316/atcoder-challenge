use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        a: [[usize; 2]; q],
    }

    let mut v = VecDeque::new();

    for i in a {
        match i[0] {
            1 => v.push_front(i[1]),
            2 => v.push_back(i[1]),
            3 => println!("{}", v[i[1] - 1]),
            _ => (),
        }
        println!("v: {:?}", v);
    }
}
