use std::io::{self, Read};

fn pow_arr(arr: &[usize], qs: &[([usize; 2], usize)]) -> Vec<u64> {
    let mut answers = vec![0u64; qs.len()];
    let mut counters = vec![0usize; 1_000_001];

    let mut curr_l = 0;
    let mut curr_r = 0;
    let mut current_answer = 0u64;

    for (q, ind) in qs {
        let mut add = |i: usize, counters: &mut Vec<usize>| {
            let x = arr[i];
            let k_m1 = counters[x];
            counters[x] += 1;
            let k = counters[x];
            current_answer = current_answer.saturating_sub((k_m1 * k_m1 * x) as u64);
            current_answer += (k * k * x) as u64;
        };
        while curr_l > q[0] {
            curr_l = curr_l.saturating_sub(1);
            add(curr_l, &mut counters);
        }
        while curr_r <= q[1] {
            add(curr_r, &mut counters);
            curr_r += 1;
        }

        let mut remove = |i: usize, counters: &mut Vec<usize>| {
            let x = arr[i];
            let k_p1 = counters[x];
            counters[x] = counters[x].saturating_sub(1);
            let k = counters[x];
            current_answer -= (k_p1 * k_p1 * x) as u64;
            current_answer += (k * k * x) as u64;
        };

        while curr_l < q[0] {
            remove(curr_l, &mut counters);
            curr_l += 1;
        }

        while curr_r > q[1] + 1 {
            curr_r = curr_r.saturating_sub(1);
            remove(curr_r, &mut counters);
        }

        answers[*ind] = current_answer;
    }

    answers
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.lines();
    let mut itr_nt = itr.next().unwrap().split_whitespace();
    let n: usize = itr_nt.next().unwrap().parse().unwrap();
    let t: usize = itr_nt.next().unwrap().parse().unwrap();

    let mut arr = vec![0usize; n + 1];
    let mut qs = vec![([0usize; 2], 0usize); t];
    for (ind, el) in itr.next().unwrap().split_whitespace().enumerate() {
        arr[ind + 1] = el.parse().unwrap();
    }
    for ind in 0..t {
        let mut itr_q = itr.next().unwrap().split_whitespace();
        qs[ind].0[0] = itr_q.next().unwrap().parse().unwrap();
        qs[ind].0[1] = itr_q.next().unwrap().parse().unwrap();
        qs[ind].1 = ind;
    }
    let bs = (n as f64).sqrt() as usize + 1;
    qs.sort_by_key(|(r, _ind)| (r[0] / bs, r[1]));
    let answers = pow_arr(&arr, &qs);

    for answ in answers {
        println!("{answ}");
    }
}
