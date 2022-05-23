use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [[usize; 2]; n],
        q: usize,
        lr: [[usize; 2]; q],
    }

    let mut first_score = vec![0];
    let mut second_score = vec![0];
    for (i, cpi) in cp.iter().enumerate() {
        if cpi[0] == 1 {
            first_score.push(cpi[1] + first_score[i]);
            second_score.push(second_score[i]);
        } else {
            first_score.push(first_score[i]);
            second_score.push(cpi[1] + second_score[i]);
        }
    }

    for j in lr {
        println!(
            "{} {}",
            first_score[j[1]] - first_score[j[0] - 1],
            second_score[j[1]] - second_score[j[0] - 1]
        )
    }
}
