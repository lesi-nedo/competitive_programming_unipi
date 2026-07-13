use std::cmp::max;
use std::io::{self, Read};

#[derive(Debug)]
struct FenwickTree {
    tree: Vec<usize>,
    len: usize,
}

impl FenwickTree {
    fn with_len(n: usize) -> Self {
        Self {
            tree: vec![0; n],
            len: n,
        }
    }

    fn lowbit(i: usize) -> usize {
        i & i.wrapping_neg()
    }

    fn query(&self, mut i: usize) -> usize {
        assert!(i < self.len);
        if i == 0 {
            return 0;
        }
        let mut res = 0usize;
        while i > 0 {
            res += self.tree[i];
            i -= Self::lowbit(i);
        }
        res
    }
    fn update(&mut self, mut i: usize, add: bool) {
        assert!(i < self.len, "I: {i} --- LEN: {}", self.len);
        let add_fun = |x: usize| x.saturating_add(1);
        let rem_fun = |x: usize| x.saturating_sub(1);
        let upd_fun = if add { add_fun } else { rem_fun };
        while i < self.len {
            self.tree[i] = upd_fun(self.tree[i]);
            i += Self::lowbit(i);
        }
    }
}

fn dfs(
    v: usize,
    p: usize,
    ind: &mut usize,
    tin: &mut [usize],
    tout: &mut [usize],
    euler: &mut [usize],
    adj: &[Vec<usize>],
) {
    tin[v] = *ind;
    euler[*ind] = v;
    *ind += 1;
    for &c in &adj[v] {
        if c == p {
            continue;
        }
        dfs(c, v, ind, tin, tout, euler, adj);
    }

    tout[v] = *ind - 1;
}

fn k_or_more(
    e: &[usize],
    c: &[usize],
    qs: &[((usize, usize), usize, usize)],
    n: usize,
    max_color: usize,
) -> Vec<usize> {
    let last = n.saturating_sub(1);

    let mut cnt_color = vec![0usize; max_color + 1];
    let mut cnt_freq = FenwickTree::with_len(n);
    let mut answers: Vec<usize> = vec![0usize; qs.len()];

    let mut cur_l = 1usize;
    let mut cur_r = 1usize;

    for &(q, k, cur_i) in qs {
        if k > n {
            answers[cur_i] = 0;
            continue;
        }
        let k_m1 = k.saturating_sub(1);
        let mut add = |i: usize| {
            let color = c[e[i]];
            let old = cnt_color[color];
            if old > 0 {
                cnt_freq.update(old, false);
            }
            let new = old + 1;
            cnt_color[color] = new;
            cnt_freq.update(new, true);
        };

        while cur_l > q.0 {
            cur_l = cur_l.saturating_sub(1);
            add(cur_l);
        }
        while cur_r <= q.1 {
            add(cur_r);
            cur_r = cur_r.saturating_add(1);
        }

        let mut remove = |i: usize| {
            let color = c[e[i]];
            let old = cnt_color[color];
            let new = old.saturating_sub(1);
            cnt_freq.update(old, false);
            if new > 0 {
                cnt_freq.update(new, true);
            }
            cnt_color[color] = new
        };

        while cur_l < q.0 {
            remove(cur_l);
            cur_l = cur_l.saturating_add(1);
        }

        while cur_r > q.1 + 1 {
            cur_r = cur_r.saturating_sub(1);
            remove(cur_r);
        }
        if cur_l == q.0 && cur_r == q.1 + 1 {
            answers[cur_i] = cnt_freq.query(last) - cnt_freq.query(k_m1);
        }
    }
    answers
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.lines();
    let mut itr_nm = itr.next().unwrap().split_whitespace();
    let mut itr_colrs = itr.next().unwrap().split_whitespace();
    let n: usize = itr_nm.next().unwrap().parse().unwrap();
    let n_p1 = n + 1;
    let m: usize = itr_nm.next().unwrap().parse().unwrap();
    let mut colors = vec![0usize; n_p1];
    let mut max_color = 0;
    for pos in colors.iter_mut().skip(1) {
        *pos = itr_colrs.next().unwrap().parse().unwrap();
        max_color = max(*pos, max_color);
    }
    let mut adj_l = vec![Vec::<usize>::new(); n_p1];
    for _ in 0..n - 1 {
        let mut itr_e = itr.next().unwrap().split_whitespace();
        let v1: usize = itr_e.next().unwrap().parse().unwrap();
        let v2: usize = itr_e.next().unwrap().parse().unwrap();
        adj_l[v1].push(v2);
        adj_l[v2].push(v1);
    }
    let mut tin = vec![0usize; n_p1];
    let mut tout = vec![0usize; n_p1];
    let mut euler = vec![0usize; n_p1];
    let mut ind = 1usize;
    dfs(1, 0, &mut ind, &mut tin, &mut tout, &mut euler, &adj_l);

    let block_size = (n as f64).sqrt() as usize + 1;
    let mut sorted_queries = vec![((0usize, 0usize), 0usize, 0usize); m];
    for ind in 0..m {
        let mut itr_q = itr.next().unwrap().split_whitespace();
        let v: usize = itr_q.next().unwrap().parse().unwrap();
        sorted_queries[ind] = (
            (tin[v], tout[v]),
            itr_q.next().unwrap().parse().unwrap(),
            ind,
        )
    }
    sorted_queries.sort_by_key(|&(q, _, _)| (q.0 / block_size, q.1));
    let answers = k_or_more(&euler, &colors, &sorted_queries, n_p1, max_color);
    for answ in answers {
        println!("{answ}");
    }
}
