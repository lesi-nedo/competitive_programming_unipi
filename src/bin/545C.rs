use std::cmp::min;
use std::io;

fn woodcutters(arr: &[[i64; 2]]) -> usize {
    let mut res = min(2, arr.len());
    let mut lfs = arr[0][0];

    for ind in 1..arr.len().saturating_sub(1) {
        let x = arr[ind][0];
        let h = arr[ind][1];
        let fl = x - h;
        let fr = x + h;
        let mut tm = x;
        if fl > lfs {
            res += 1;
        } else if fr < arr[ind + 1][0] {
            tm = fr;
            res += 1;
        }
        lfs = tm;
    }

    res
}

fn main() {
    let mut lines = io::stdin().lines();
    loop {
        let n: usize;
        match lines.next() {
            Some(Ok(sn)) if sn.trim().is_empty() => continue,
            Some(Ok(sn)) => n = sn.parse().unwrap(),
            Some(Err(e)) => {
                eprintln!("Error in processing n: {e}");
                break;
            }
            None => break,
        }
        let mut ts: Vec<[i64; 2]> = Vec::with_capacity(n);
        for _ in 0..n {
            match lines.next() {
                Some(Ok(c)) if c.trim().is_empty() => continue,
                Some(Ok(c)) => {
                    let mut itr = c.split_whitespace();
                    let x: i64 = itr.next().unwrap().parse().unwrap();
                    let h: i64 = itr.next().unwrap().parse().unwrap();
                    ts.push([x, h]);
                }
                Some(Err(e)) => {
                    eprintln!("Error in parsing tree coordinates: {e}");
                    break;
                }
                None => break,
            }
        }
        println!("{}", woodcutters(&ts));
    }
}
