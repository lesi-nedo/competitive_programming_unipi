use std::cmp::Ordering;
use std::cmp::max;
use std::collections::BTreeSet;
use std::io;

#[derive(Eq, PartialEq)]
struct Item {
    val: [i64; 2],
    id: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val[0].cmp(&other.val[0]).then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct STree {
    n: usize,
    tree: Vec<i64>,
}

impl STree {
    fn new(keys: &Vec<i64>) -> Self {
        let n = keys.len();
        let mut st = STree {
            n,
            tree: vec![i64::MAX; 4 * n],
        };
        st.build(1, 0, n - 1, keys);
        st
    }
    fn left(&self, node: usize) -> usize {
        node * 2
    }
    fn right(&self, node: usize) -> usize {
        node * 2 + 1
    }

    fn build(&mut self, node: usize, l: usize, r: usize, keys: &Vec<i64>) {
        if l == r {
            self.tree[node] = keys[l];
        } else {
            let m = (l + r) / 2;
            let left_node = self.left(node);
            let right_node = self.right(node);
            self.build(left_node, l, m, keys);
            self.build(right_node, m + 1, r, keys);
            self.tree[node] = max(self.tree[left_node], self.tree[right_node]);
        }
    }

    // pub fn print(&self) {
    //     self.print_rec(1, 0, self.n - 1, 0);
    // }
    //
    // fn print_rec(&self, node: usize, l: usize, r: usize, depth: usize) {
    //     // indentation proportional to depth
    //     let indent = "  ".repeat(depth);
    //
    //     println!(
    //         "{}[node {}] range=({}, {}) val={}",
    //         indent,
    //         node,
    //         l,
    //         r,
    //         self.tree[node]
    //     );
    //
    //     if l == r {
    //         return;
    //     }
    //
    //     let mid = (l + r) / 2;
    //     self.print_rec(self.left(node), l, mid, depth + 1);
    //     self.print_rec(self.right(node), mid + 1, r, depth + 1);
    // }

    fn update(&mut self, node: usize, l: usize, r: usize, idx: usize, val: i64) {
        if l == r {
            self.tree[node] = val;
        } else {
            let m = (l + r) / 2;
            let (left, right) = (self.left(node), self.right(node));
            if idx <= m {
                self.update(left, l, m, idx, val);
            } else {
                self.update(right, m + 1, r, idx, val);
            }
            self.tree[node] = max(self.tree[left], self.tree[right]);
        }
    }

    fn query(&self, node: usize, l: usize, r: usize, qr: usize, x: i64) -> Option<usize> {
        if l > qr || self.tree[node] < x {
            return None;
        }
        if l == r {
            return Some(l);
        }
        let mid = (l + r) / 2;

        if let Some(i) = self.query(self.left(node), l, mid, qr, x) {
            return Some(i);
        }
        self.query(self.right(node), mid + 1, r, qr, x)
    }
}

fn load_data<T>(data: &mut [[T; 2]], n: usize)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.split_whitespace();
        for j in 0..2 {
            data[i][j] = it.next().unwrap().parse::<T>().unwrap();
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let mut frogs = vec![[0i64; 2]; n];
    let mut mosquitoes = vec![[0i64; 2]; m];
    load_data(&mut frogs, n);
    load_data(&mut mosquitoes, m);
    let mut frogs_with_pos: Vec<([i64; 2], usize, (i64, i64))> = frogs
        .into_iter()
        .enumerate()
        .map(|(i, [x, l])| ([x, l], i, (0, l)))
        .collect();
    frogs_with_pos.sort_by_key(|(p, _, _)| p[0]);
    let xs: Vec<i64> = frogs_with_pos.iter().map(|(p, _, _)| p[0]).collect();
    let mut ds: Vec<i64> = frogs_with_pos.iter().map(|(p, _, _)| p[1]).collect();
    let keys: Vec<i64> = xs.iter().zip(&ds).map(|(&x, &d)| x + d).collect();
    let mut st = STree::new(&keys);
    let mut tset: BTreeSet<Item> = BTreeSet::new();

    for (id, m) in mosquitoes.iter().enumerate() {
        let mp = m[0];
        let pos_op = match xs.binary_search(&mp) {
            Ok(ind) => Some(ind),
            Err(ind) if ind > 0 => Some(ind - 1),
            _ => None,
        };
        tset.insert(Item { val: *m, id });
        if let Some(pos) = pos_op {
            let r = if pos == xs.len() || xs[pos] > mp {
                pos - 1
            } else {
                pos
            };
            if let Some(pf) = st.query(1, 0, st.n - 1, r, mp) {
                loop {
                    let low = Item {
                        val: [xs[pf], 0],
                        id: 0,
                    };
                    let high = Item {
                        val: [ds[pf] + xs[pf], 0],
                        id: usize::MAX,
                    };
                    let to_remove: Vec<_> = tset
                        .range(low..=high)
                        .map(|m| (m.val, m.id))
                        .inspect(|(val, _)| {
                            ds[pf] += val[1];
                            frogs_with_pos[pf].2.0 += 1;
                            frogs_with_pos[pf].2.1 = ds[pf];
                        })
                        .collect();
                    if to_remove.is_empty() {
                        break;
                    }
                    for (val, id) in to_remove {
                        tset.remove(&Item { val, id });
                    }
                }
                st.update(1, 0, st.n - 1, pf, ds[pf] + xs[pf]);
            }
        }
    }

    frogs_with_pos.sort_by_key(|(_, ind, _)| *ind);
    for (_, _, r) in frogs_with_pos {
        println!("{} {}", r.0, r.1);
    }
}
