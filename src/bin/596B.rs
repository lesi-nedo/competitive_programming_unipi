use std::io;

fn wilbur_and_array(arr: &[i64]) -> u64 {
    let mut res = 0;
    let mut delta = 0i64;

    for &el in arr {
        let temp = el - delta;
        delta += temp;
        res += temp.unsigned_abs();
    }

    res
}

fn main() {
    let mut lines = io::stdin().lines();

    loop {
        let n: usize;
        match lines.next() {
            Some(Ok(ns)) => {
                n = ns.parse().unwrap();
            }
            Some(Err(e)) => {
                eprintln!("Error in getting n: {e}");
                break;
            }
            None => break,
        }

        let mut bs: Vec<i64> = Vec::with_capacity(n);
        match lines.next() {
            Some(Ok(nums)) => {
                bs.extend(nums.split_whitespace().map(|n| n.parse::<i64>().unwrap()));
                println!("{}", wilbur_and_array(&bs));
            },
            Some(Err(e)) => {
                eprintln!("Error in reading the array b: {e}");
                break;
            },
            None => break,
        }
    }
}
