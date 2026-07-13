use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct SegTree {
    tree: Vec<usize>,
    last: usize,
}

impl SegTree {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            tree: vec![0; 2 * n],
            last: n.saturating_sub(1),
        }
    }
    fn update_util(&mut self, si: usize, ss: usize, se: usize, ui: usize, uv: usize) -> usize {
        if ss == se && se == ui {
            self.tree[si] = max(self.tree[si], uv);
            return self.tree[si];
        }
        if ss > se || ss > ui || ui > se {
            return self.tree[si];
        }
        let md = Self::gm(ss, se);
        let lc = Self::glc(si);
        let rc = Self::grc(si, ss, md);
        let lv = self.update_util(lc, ss, md, ui, uv);
        let rv = self.update_util(rc, md + 1, se, ui, uv);
        self.tree[si] = max(lv, rv);
        self.tree[si]
    }
    pub fn update(&mut self, ui: usize, uv: usize) {
        assert!(ui <= self.last);
        self.update_util(0, 0, self.last, ui, uv);
    }
    fn query_util(&self, si: usize, ss: usize, se: usize, qs: usize, qe: usize) -> usize {
        if ss >= qs && se <= qe {
            return self.tree[si];
        }
        if ss > se || ss > qe || se < qs {
            return 0;
        }
        let md = Self::gm(ss, se);
        let lc = Self::glc(si);
        let rc = Self::grc(si, ss, md);
        max(
            self.query_util(lc, ss, md, qs, min(qe, md)),
            self.query_util(rc, md + 1, se, max(qs, md + 1), qe),
        )
    }

    pub fn query(&self, qs: usize, qe: usize) -> usize {
        assert!(qs <= qe);
        self.query_util(0, 0, self.last, qs, qe)
    }
    fn gm(s: usize, e: usize) -> usize {
        s + (e - s) / 2
    }
    fn glc(si: usize) -> usize {
        si + 1
    }
    fn grc(si: usize, ss: usize, sm: usize) -> usize {
        si + 2 * (sm - ss + 1)
    }
}

fn lis(arr: &Vec<usize>) -> usize {
    let mut c_arr: Vec<usize> = arr.clone();
    c_arr.sort_unstable();
    c_arr.dedup();
    let cmp: HashMap<usize, usize> = c_arr.iter().enumerate().map(|(i, &val)| (val, i)).collect();
    let mut re = 0usize;
    let mut sgt = SegTree::with_capacity(c_arr.len());
    for x in arr {
        let c_x = *cmp.get(x).unwrap();
        let mut c = 1;
        if c_x != 0 {
            c += sgt.query(0, c_x.saturating_sub(1));
        }
        re = max(re, c);
        sgt.update(c_x, c);
    }

    re
}

fn main() {
    for line in io::stdin().lines() {
        match line {
            Ok(txt) => {
                let arr: Vec<usize> = txt
                    .split_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect();
                let res = lis(&arr);
                println!("{res}");
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
}
