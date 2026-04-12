use std::io::{self, Read};

fn sol_ilya_q(s: &str, queries: &[[usize; 2]]) {
    let chars: Vec<char> = s.chars().collect();
    let mut prefix_sum = vec![0usize; chars.len()];

    for ind in 0..chars.len()-1 {
        prefix_sum[ind] = if chars[ind] == chars[ind+1] { 1 } else { 0 };

        if ind > 0 {
            prefix_sum[ind] += prefix_sum[ind-1];
        }
    }

    for q in queries {
        let s = prefix_sum[q[1]-2];
        let f = if q[0] > 1 { prefix_sum[q[0]-2] } else { 0 };

        println!("{}", s - f);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let s = it.next().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut queries = vec![];

    for _ in 0..m {
        let l = it.next().unwrap().parse().unwrap();
        let r = it.next().unwrap().parse().unwrap();
        queries.push([l, r]);
    }

    sol_ilya_q(s, &queries);
}