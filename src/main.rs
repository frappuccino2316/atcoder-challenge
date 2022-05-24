use proconio::input;

fn main() {
    input! {
        n: i64,
        s: [String; n],
    }

    let mut answer = Vec::new();

    for (i, e) in s.iter().enumerate() {
        if answer.is_empty() || answer.iter().all(|x| *x != e) {
            println!("{}", i + 1);
            answer.push(e);
        }
    }
}
