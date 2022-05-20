use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let r = if (a * b) % 2 == 0 {
        String::from("Even")
    } else {
        String::from("Odd")
    };
    println!("{}", r);
}
