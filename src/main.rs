use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [String; h],
    }

    let mut first = true;
    let mut r = [0, 0];

    for (i, row) in s.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == 'o' {
                if first {
                    r[0] += i as i32;
                    r[1] += j as i32;
                    first = false;
                } else {
                    r[0] -= i as i32;
                    r[1] -= j as i32;
                }
            }
        }
    }

    println!("{}", r[0].abs() + r[1].abs());
}
