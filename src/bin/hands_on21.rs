use std::cmp::{max, min};
use std::io::{self, Read};

#[derive(Debug)]
struct SegTree {
    tree: Vec<usize>,
    last: usize,
}

impl SegTree {
    fn build(arr: &[usize]) -> Self {
        let s = arr.len() * 2;
        let mut sg = Self {
            tree: vec![0usize; s],
            last: arr.len().saturating_sub(1),
        };
        sg.build_util(arr, 0, 0, sg.last);
        sg
    }

    fn build_util(&mut self, arr: &[usize], i: usize, l: usize, r: usize) -> usize {
        if l == r {
            self.tree[i] = arr[l];
            return arr[l];
        }
        let md = Self::gm(l, r);
        let l_v = self.build_util(arr, Self::glc(i), l, md);
        let r_v = self.build_util(arr, Self::grc(i, l, md), md + 1, r);
        self.tree[i] = max(l_v, r_v);
        self.tree[i]
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
    fn max(&self, ql: usize, qr: usize) -> usize {
        let ql = ql.saturating_sub(1);
        let qr = qr.saturating_sub(1);
        assert!(ql <= qr);
        assert!(qr <= self.last);
        self.max_util(0, 0, self.last, ql, qr)
    }
    fn max_util(&self, i: usize, l: usize, r: usize, ql: usize, qr: usize) -> usize {
        if ql <= l && r <= qr {
            return self.tree[i];
        }
        if l > r || ql > r || qr < l {
            return 0;
        }
        let md = Self::gm(l, r);
        let l_val = self.max_util(Self::glc(i), l, md, ql, min(qr, md));
        let r_val = self.max_util(Self::grc(i, l, md), md + 1, r, max(ql, md + 1), qr);
        max(l_val, r_val)
    }
    fn update(&mut self, ul: usize, ur: usize, v: usize) {
        let ul = ul.saturating_sub(1);
        let ur = ur.saturating_sub(1);
        assert!(ul <= ur);
        assert!(ur <= self.last);

        self.update_util(0, 0, self.last, ul, ur, v);
    }
    fn update_util(
        &mut self,
        i: usize,
        l: usize,
        r: usize,
        ul: usize,
        ur: usize,
        v: usize,
    ) -> usize {
        if l == r && ul <= l && l <= ur {
            // println!("L: {l} ---- UL: {ul} --- UR: {ur}");
            self.tree[i] = min(v, self.tree[i]);
            self.tree[i]
        } else {
            if l > r || l > ur || r < ul {
                return self.tree[i];
            }
            let md = Self::gm(l, r);
            let l_v = self.update_util(Self::glc(i), l, md, ul, min(md, ur), v);
            let r_v = self.update_util(Self::grc(i, l, md), md + 1, r, max(ul, md + 1), ur, v);
            self.tree[i] = max(l_v, r_v);
            self.tree[i]
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.lines();
    let mut itr_nm = itr.next().unwrap().split_whitespace();
    let n: usize = itr_nm.next().unwrap().parse().unwrap();
    let mut arr = vec![0usize; n];
    for (ind, el) in itr.next().unwrap().split_whitespace().enumerate() {
        arr[ind] = el.parse().unwrap();
    }
    let mut sg = SegTree::build(&arr);
    for line in itr {
        let mut itr = line.split_whitespace();
        let qr: bool = itr.next().unwrap() == "1";
        match qr {
            true => println!(
                "{}",
                sg.max(
                    itr.next().unwrap().parse().unwrap(),
                    itr.next().unwrap().parse().unwrap()
                )
            ),
            false => sg.update(
                itr.next().unwrap().parse().unwrap(),
                itr.next().unwrap().parse().unwrap(),
                itr.next().unwrap().parse().unwrap(),
            ),
        }
    }
}
