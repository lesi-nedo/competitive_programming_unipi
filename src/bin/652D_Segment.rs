use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct SegTree {
    tree: Vec<usize>,
}

impl SegTree {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            tree: vec![0; 2 * n],
        }
    }
    pub fn query(&self, ss: usize, se: usize, sq: usize, eq: usize, si: usize) -> usize {
        if sq <= ss && se <= eq {
            return self.tree[si];
        }

        if ss > eq || se < sq {
            return 0;
        }

        let mid = Self::get_mid(ss, se);
        self.query(ss, mid, sq, eq, Self::get_left_child(si))
            + self.query(mid + 1, se, sq, eq, Self::get_right_child(si, ss, mid))
    }

    pub fn increment_by_1(&mut self, ss: usize, se: usize, ii: usize, si: usize) {
        if ii < ss || ii > se {
            return;
        }
        self.tree[si] += 1;

        if ss != se {
            let mid = Self::get_mid(ss, se);
            self.increment_by_1(ss, mid, ii, Self::get_left_child(si));
            self.increment_by_1(mid + 1, se, ii, Self::get_right_child(si, ss, mid));
        }
    }
    fn get_left_child(n: usize) -> usize {
        n + 1
    }
    fn get_right_child(n: usize, s: usize, mid: usize) -> usize {
        n + 2 * (mid - s + 1)
    }
    fn get_mid(s: usize, e: usize) -> usize {
        s + (e - s) / 2
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let mut sg = vec![([0i64; 2], 0usize); n];
    for ind in 0..n {
        sg[ind].0[0] = itr.next().unwrap().parse().unwrap();
        sg[ind].0[1] = itr.next().unwrap().parse().unwrap();
        sg[ind].1 = ind;
    }
    let mut res = vec![0usize; sg.len()];
    let mut coords: Vec<i64> = sg.iter().flat_map(|(s, _)| *s).collect();
    coords.sort();
    coords.dedup();
    sg.sort_unstable_by_key(|(s, _)| Reverse(s[0]));

    let mut map = HashMap::with_capacity(coords.len());
    for (ind, r) in coords.iter().enumerate() {
        map.insert(r, ind);
    }
    let mut tr = SegTree::with_capacity(coords.len());
    let n = coords.len().saturating_sub(1);

    for (s, ind) in sg {
        res[ind] = tr.query(
            0,
            n,
            map.get(&s[0]).unwrap() + 1,
            map.get(&s[1]).unwrap().saturating_sub(1),
            0,
        );
        tr.increment_by_1(0, n, *map.get(&s[1]).unwrap(), 0);
    }
    for r in res {
        println!("{r}");
    }
}
