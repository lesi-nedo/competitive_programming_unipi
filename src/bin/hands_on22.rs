use std::cmp::{max, min};
use std::io::{self, Read};

#[derive(Debug, Clone)]
struct Node {
    min_max: [usize; 2],
}

impl Node {
    fn new() -> Self {
        Self {
            min_max: [usize::MIN, usize::MIN],
        }
    }
}

#[derive(Debug)]
struct SegTree {
    tree: Vec<Node>,
    last: usize,
    lazy: Vec<usize>,
}

impl SegTree {
    fn build(arr: &[[usize; 2]]) -> Self {
        let n = arr.len() * 2;
        let mut st = SegTree {
            tree: vec![Node::new(); n],
            last: arr.len().saturating_sub(1),
            lazy: vec![0; n],
        };
        for s in arr {
            st.build_util(0, 0, st.last, s[0], s[1]);
        }
        st
    }

    fn push(&mut self, i: usize, left: usize, right: usize) {
        let delta = self.lazy[i];
        self.increment(left, delta);
        self.lazy[left] += delta;
        self.increment(right, delta);
        self.lazy[right] += delta;

        self.lazy[i] = 0;
    }

    fn increment(&mut self, i: usize, delta: usize) {
        self.tree[i].min_max[0] += delta;
        self.tree[i].min_max[1] += delta;
    }
    fn build_util(&mut self, i: usize, l: usize, r: usize, ss: usize, se: usize) {
        if ss > se || l > se || r < ss {
            return;
        }
        if l == ss && r == se {
            self.increment(i, 1);
            self.lazy[i] += 1
        } else {
            let md = Self::gm(l, r);
            let left_c = Self::glc(i);
            let right_c = Self::grc(i, l, md);
            self.push(i, left_c, right_c);
            self.build_util(left_c, l, md, ss, min(se, md));
            self.build_util(right_c, md + 1, r, max(ss, md + 1), se);
            self.tree[i].min_max[0] =
                min(self.tree[left_c].min_max[0], self.tree[right_c].min_max[0]);
            self.tree[i].min_max[1] =
                max(self.tree[left_c].min_max[1], self.tree[right_c].min_max[1]);
        }
    }
    fn is_there(&mut self, ql: usize, qr: usize, k: usize) -> usize {
        assert!(ql <= qr);
        assert!(qr <= self.last);
        self.is_there_util(0, 0, self.last, ql, qr, k)
    }
    fn is_there_util(
        &mut self,
        i: usize,
        l: usize,
        r: usize,
        ql: usize,
        qr: usize,
        k: usize,
    ) -> usize {
        if ql > qr || l > qr || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            let min_max = self.tree[i].min_max;
            if min_max[0] <= k && k <= min_max[1] {
                return 1;
            }
            return 0;
        }
        let md = Self::gm(l, r);
        let left_c = Self::glc(i);
        let right_c = Self::grc(i, l, md);
        self.push(i, left_c, right_c);
        max(
            self.is_there_util(left_c, l, md, ql, min(qr, md), k),
            self.is_there_util(right_c, md + 1, r, max(ql, md + 1), qr, k),
        )
    }
    fn gm(l: usize, r: usize) -> usize {
        l + (r - l) / 2
    }
    fn glc(i: usize) -> usize {
        i + 1
    }
    fn grc(i: usize, l: usize, m: usize) -> usize {
        i + 2 * (m - l + 1)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.lines();
    let mut itr_nm = itr.next().unwrap().split_whitespace();
    let n: usize = itr_nm.next().unwrap().parse().unwrap();
    let m: usize = itr_nm.next().unwrap().parse().unwrap();
    let mut sg = vec![[0usize; 2]; n];
    for ind in 0..n {
        let mut itr_n = itr.next().unwrap().split_whitespace();
        sg[ind][0] = itr_n.next().unwrap().parse().unwrap();
        sg[ind][1] = itr_n.next().unwrap().parse().unwrap();
    }
    let mut st = SegTree::build(&sg);
    for _ in 0..m {
        let mut q = itr.next().unwrap().split_whitespace();
        let res = st.is_there(
            q.next().unwrap().parse().unwrap(),
            q.next().unwrap().parse().unwrap(),
            q.next().unwrap().parse().unwrap(),
        );
        println!("{res}");
    }
}
