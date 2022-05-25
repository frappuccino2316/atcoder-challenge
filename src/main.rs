use proconio::input;

fn main() {
    input! {
        n: String,
        k: i64,
    }

    // 入力された数字を1の位から並ぶVecに変換
    let mut n_eight = Vec::new();
    for i in n.chars() {
        n_eight.push(i as i64 - 48);
    }
    n_eight.reverse();

    let mut ans = String::from("");

    // 「8進数を9進数に変換 & 8を5に置き換える」という操作をk回行う
    for j in 0..k {
        let mut n_nine = Vec::new();
        let mut n_ten = 0;

        // 10進数に変換
        for (m, v) in n_eight.iter().enumerate() {
            n_ten += 8_i64.pow(m as u32) * v;
        }

        // 9進数に変換して、n_nineに1の位からのVecとして格納
        loop {
            println!("loop");
            let x = n_ten % 9;
            n_nine.push(x);
            n_ten /= 9;
            if n_ten == 0 {
                break;
            }
        }
        // 8があれば5に変換
        for (_, e) in n_nine.iter_mut().enumerate() {
            if *e == 8 {
                *e = 5;
            }
        }
        // 次のループのために、操作したものを新たな8進数の値とする
        n_eight = n_nine;

        // 最後のループなら文字列に結合する
        if j == k - 1 {
            n_eight.reverse();
            let tmp: Vec<String> = n_eight.iter().map(|x| x.to_string()).collect();
            ans = tmp.join("");
        }
    }

    println!("{}", ans);
}
