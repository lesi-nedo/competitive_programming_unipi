use std::cmp::max;
use std::io;
use std::io::Read;

fn holiday_planning(n: usize, d: usize, mtr: &[Vec<usize>]) -> usize {
    let dp1 = d + 1;
    let np1 = n + 1;
    let mut mrs: Vec<Vec<usize>> = vec![vec![0; dp1]; np1];
    let mut sums = vec![vec![0usize; dp1]; np1];
    for c in 1..np1 {
        for day in 1..dp1 {
            sums[c][day] = mtr[c - 1][day - 1] + sums[c][day - 1];
        }
    }
    for c in 1..np1 {
        for j in 1..dp1 {
            for k in 0..=j {
                mrs[c][j] = max(mrs[c][j], mrs[c - 1][j - k] + sums[c][k]);
            }
        }
    }
    mrs[n][d]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut itr = lines.next().unwrap().split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let d: usize = itr.next().unwrap().parse().unwrap();
    let mut mtr: Vec<Vec<usize>> = vec![vec![0; d]; n];
    for (ind_n, line) in lines.enumerate() {
        itr = line.split_whitespace();
        for (ind_d, d) in itr.enumerate() {
            mtr[ind_n][ind_d] = d.parse().unwrap();
        }
    }

    println!("{}", holiday_planning(n, d, &mtr));
}
