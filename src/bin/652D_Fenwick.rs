use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, Default, Clone, Copy)]
struct Segment {
    l: i64,
    r: i64,
    id: usize,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Segment {}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id == other.id {
            return Ordering::Equal;
        }

        self.r.cmp(&other.r).then_with(|| self.l.cmp(&other.l))
    }
}

#[derive(Debug)]
struct FenwickTree {
    tree: Vec<usize>,
}

impl FenwickTree {
    fn with_len(n: usize) -> Self {
        Self {
            tree: vec![usize::default(); n + 1],
        }
    }

    fn sibling(i: usize) -> usize {
        i | (i + 1)
    }
    fn parent(i: usize) -> usize {
        (i & (i + 1)).saturating_sub(1)
    }
    fn query(&self, i: usize) -> usize {
        assert!(self.tree.len() > i);
        let mut i = i + 1;
        let mut res = 0;
        while i > 0 {
            res += self.tree[i];
            i = Self::parent(i);
        }

        res
    }

    fn increase_by_1(&mut self, i: usize) {
        assert!(i < self.tree.len());
        let mut i = i + 1;
        while i < self.tree.len() {
            self.tree[i] += 1;
            i = Self::sibling(i);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let mut sg = vec![Segment::default(); n];
    let mut res = vec![usize::default(); n];
    let mut le = vec![0i64; n];
    for s in 0..n {
        sg[s].l = itr.next().unwrap().parse().unwrap();
        sg[s].r = itr.next().unwrap().parse().unwrap();
        sg[s].id = s;
        le[s] = sg[s].l
    }
    le.sort_unstable_by(|a, b| b.cmp(a));
    let map: HashMap<i64, usize> = le.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
    sg.sort_unstable_by(|s1, s2| s1.r.cmp(&s2.r).then_with(|| s1.l.cmp(&s2.l)));

    let mut tree = FenwickTree::with_len(n);
    for s in sg {
        let rank = *map.get(&s.l).unwrap();
        res[s.id] = tree.query(rank);
        tree.increase_by_1(rank);
    }

    for r in res {
        println!("{r}");
    }
}
