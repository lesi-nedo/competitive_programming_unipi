use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug)]
struct Fenwick<T> {
    tree: Vec<T>,
}

impl<T> Fenwick<T> {
    fn parent(i: usize) -> usize {
        (i & (i + 1)).saturating_sub(1)
    }

    fn sibling(i: usize) -> usize {
        i | (i + 1)
    }
}

pub trait Tree {
    type Item;
    fn update(&mut self, i: usize, v: Self::Item);
    fn query(&self, i: usize) -> Self::Item;
    fn with_len(n: usize) -> Self;
    fn len(&self) -> usize;
}

impl<T> Tree for Fenwick<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Copy
        + Clone
        + Default
        + AddAssign,
{
    type Item = T;
    fn update(&mut self, i: usize, v: Self::Item) {
        let mut i = i;
        assert!(i < self.tree.len());
        while i < self.tree.len() {
            self.tree[i] += v;
            i = Self::sibling(i);
        }
    }
    fn query(&self, i: usize) -> Self::Item {
        let mut i = i;
        assert!(i < self.tree.len());
        let mut sum = Self::Item::default();
        while i > 0 {
            sum += self.tree[i];
            i = Self::parent(i);
        }
        sum
    }

    fn with_len(n: usize) -> Self {
        Self {
            tree: vec![Self::Item::default(); n + 1],
        }
    }
    fn len(&self) -> usize {
        self.tree.len().saturating_sub(1)
    }
}

fn f_sol(nums: &[i64]) -> usize {
    let mut res = 0usize;
    let mut fl = vec![0usize; nums.len()];
    let mut fr = vec![0usize; nums.len()];
    let mut map: HashMap<i64, usize> = HashMap::new();
    for (ind, &el) in nums.iter().enumerate() {
        let count = map.entry(el).or_insert(0);
        *count += 1;
        fl[ind] = *count;
    }
    map.clear();
    for ind in (0..nums.len()).rev() {
        let count = map.entry(nums[ind]).or_insert(0);
        *count += 1;
        fr[ind] = *count;
    }
    let mut tree: Fenwick<usize> = Fenwick::with_len(nums.len());
    let n = nums.len();
    for k in 0..nums.len() {
        res += tree.query(n).saturating_sub(tree.query(fr[k]));
        tree.update(fl[k], 1);
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    let mut nums = vec![0i64; n];
    for ind in 0..n {
        nums[ind] = itr.next().unwrap().parse().unwrap();
    }
    println!("{}", f_sol(&nums));
}
