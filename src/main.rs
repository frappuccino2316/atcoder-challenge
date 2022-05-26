use proconio::input;

// 002 - Encyclopedia of Parentheses（★3）
// https://atcoder.jp/contests/typical90/tasks/typical90_b
//
// カッコ列が正しい条件
// - 1文字目が「(」
// - 常に「(」が「)」より多い
// - 全部の「(」と「)」の数が等しい

fn main() {
    input! {
        n: usize,
    }

    for bit in 0..(1 << n) {
        let mut candidate = String::from("");

        let mut i = (n - 1) as isize;
        while i >= 0 {
            if bit & (1 << i) == 0 {
                candidate += "(";
            } else {
                candidate += ")";
            }
            i -= 1;
        }

        if check(&candidate) {
            println!("{}", candidate);
        }
    }
}

fn check(s: &str) -> bool {
    let mut depth = 0;
    for j in s.chars() {
        if j == '(' {
            depth += 1;
        } else {
            depth -= 1;
        }
        if depth < 0 {
            return false;
        }
    }
    if depth == 0 {
        return true;
    }
    false
}
