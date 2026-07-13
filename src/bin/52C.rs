use std::cmp::{max, min};
use std::fmt::{Debug, Display};
use std::io::{self, Read};
use std::ops::{Add, AddAssign};
use std::str::FromStr;

trait Max {
    const MAX: Self;
}

impl Max for i64 {
    const MAX: Self = i64::MAX;
}

#[derive(Debug)]
struct SegTree<T> {
    tree: Vec<T>,
    lazy: Vec<T>,
    fi: usize,
}

impl<T> SegTree<T>
where
    T: Default + Clone + Ord + Copy + Add<Output = T> + AddAssign + Max,
    T: Display,
{
    fn build(arr: &[T], n: usize) -> Self {
        let mut sg = Self {
            tree: vec![T::default(); 2 * n],
            lazy: vec![T::default(); 2 * n],
            fi: n.saturating_sub(1),
        };
        sg.build_util(arr, 0, n - 1, 0);
        sg
    }
    fn build_util(&mut self, arr: &[T], s: usize, e: usize, i: usize) -> T {
        if s == e {
            self.tree[i] = arr[s];
            return self.tree[i];
        }
        let mid = Self::gm(s, e);
        self.tree[i] = min(
            self.build_util(arr, s, mid, Self::glc(i)),
            self.build_util(arr, mid + 1, e, Self::grc(i, s, mid)),
        );
        self.tree[i]
    }
    fn glc(n: usize) -> usize {
        n + 1
    }
    fn grc(n: usize, s: usize, m: usize) -> usize {
        n + 2 * (m - s + 1)
    }
    fn gm(s: usize, e: usize) -> usize {
        s + (e - s) / 2
    }
    fn rmq(&mut self, lf: usize, rg: usize) -> T {
        if lf > rg {
            return min(
                self.rmq_util(0, self.fi, lf, self.fi, 0),
                self.rmq_util(0, self.fi, 0, rg, 0),
            );
        }
        self.rmq_util(0, self.fi, lf, rg, 0)
    }

    fn push(&mut self, i: usize, s: usize, m: usize) {
        let left = Self::glc(i);
        let right = Self::grc(i, s, m);
        let delta = self.lazy[i];
        self.tree[left] += delta;
        self.lazy[left] += delta;
        self.tree[right] += delta;
        self.lazy[right] += delta;

        self.lazy[i] = T::default();
    }

    fn rmq_util(&mut self, s: usize, e: usize, lf: usize, rg: usize, i: usize) -> T {
        if lf > rg {
            return T::MAX;
        }
        if s == lf && e == rg {
            return self.tree[i];
        }
        let mid = Self::gm(s, e);
        self.push(i, s, mid);

        min(
            self.rmq_util(s, mid, lf, min(rg, mid), Self::glc(i)),
            self.rmq_util(mid + 1, e, max(lf, mid + 1), rg, Self::grc(i, s, mid)),
        )
    }
    fn update(&mut self, l: usize, r: usize, val: T) {
        if l > r {
            self.update_util(0, self.fi, l, self.fi, 0, val);
            self.update_util(0, self.fi, 0, r, 0, val);
            return;
        }
        self.update_util(0, self.fi, l, r, 0, val);
    }
    fn update_util(&mut self, s: usize, e: usize, l: usize, r: usize, i: usize, val: T) {
        if l > r {
            return;
        }
        if l == s && e == r {
            self.tree[i] += val;
            self.lazy[i] += val;
        } else {
            let mid = Self::gm(s, e);
            self.push(i, s, mid);
            let left_c = Self::glc(i);
            let right_c = Self::grc(i, s, mid);
            self.update_util(s, mid, l, min(r, mid), left_c, val);
            self.update_util(mid + 1, e, max(l, mid + 1), r, right_c, val);
            self.tree[i] = min(self.tree[left_c], self.tree[right_c]);
        }
    }
}

fn parse_str_to_arr<T>(num_str: &str, arr: &mut Vec<T>)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    for sn in num_str.split_whitespace() {
        arr.push(sn.parse().unwrap());
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let parts: Vec<&str> = input.lines().collect();
    let mut arr: Vec<i64> = Vec::new();
    parse_str_to_arr(parts[1], &mut arr);
    let mut sg = SegTree::build(&arr, arr.len());
    let m: usize = parts[2].parse().unwrap();
    for el in parts.iter().skip(3).take(m) {
        let mut temp: Vec<i64> = Vec::new();
        parse_str_to_arr(el, &mut temp);
        if temp.len() == 2 {
            println!("{}", sg.rmq(temp[0] as usize, temp[1] as usize));
        } else {
            sg.update(temp[0] as usize, temp[1] as usize, temp[2]);
        }
    }
}
