use std::cmp::max;
use std::io::{self, BufRead};

fn lsc(str1: &str, str2: &str) -> usize {
    let str1 = str1.as_bytes();
    let str2 = str2.as_bytes();
    let n1 = str1.len();
    let n2 = str2.len();
    if n1 == 0 || n2 == 0 {
        return 0;
    }
    let n1p1 = n1 + 1;
    let n2p1 = n2 + 1;
    let mut mtr = vec![0usize; n1p1 * n2p1];
    for ind_str1 in 1..n1p1 {
        for ind_str2 in 1..n2p1 {
            if str1[ind_str1 - 1] == str2[ind_str2 - 1] {
                mtr[ind_str1 * n2p1 + ind_str2] = mtr[(ind_str1 - 1) * n2p1 + ind_str2 - 1] + 1;
            } else {
                mtr[ind_str1 * n2p1 + ind_str2] = max(
                    mtr[(ind_str1 - 1) * n2p1 + ind_str2],
                    mtr[ind_str1 * n2p1 + ind_str2 - 1],
                );
            }
        }
    }
    mtr[n1p1 * n2p1 - 1]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        let str1 = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => {
                eprintln!("Error reading input str1: {e}");
                break;
            }
            None => break,
        };
        let str2 = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => {
                eprintln!("Error reading input str2: {e}");
                break;
            }
            None => {
                eprintln!("Expected a second line, got EOF");
                break;
            }
        };
        let res = lsc(&str1, &str2);
        println!("{res}");
    }
}
