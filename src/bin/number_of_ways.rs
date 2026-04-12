use std::io::{self, Read};

fn n_ways(els: &[i64]) -> usize {
    let mut res: usize = 0;
    let els_n = els.iter().len().saturating_sub(1);
    if els_n <= 1 {
        return res;
    }

    let sums = els.iter().scan(0, |sum, e|
        {
            *sum += e;
            Some(*sum)
        }).collect::<Vec<_>>();
    let fin = els_n - 1;
    let scn = els_n;
    for i in 0..fin {
        for j in i+1..scn {
            let fir = sums[i];
            let scr = sums[j] - fir;
            let thr = sums[els_n] - sums[j];
            if fir == scr && scr == thr {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut els = vec![0i64; n];
    for ind in 0..n {
        els[ind] = it.next().unwrap().parse().unwrap();
    }

    println!("{}", n_ways(&els));
}