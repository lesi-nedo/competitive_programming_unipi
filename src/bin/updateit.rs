use std::io::{self, Read};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

pub trait Tree {
    type Item;
    fn with_len(n: usize) -> Self;
    fn len(&self) -> usize;
    fn add(&mut self, i: usize, delta: Self::Item);
    fn sum(&self, i: usize) -> Self::Item;
    fn range_sum(&self, l: usize, r: usize) -> Self::Item;
}

#[derive(Debug)]
struct Fenwick<T> {
    tree: Vec<T>,
}

impl<T> Fenwick<T> {
    fn isolate_trailing_one(i: usize) -> usize {
        if i == 0 { 0 } else { 1 << i.trailing_zeros() }
    }

    fn parent(i: usize) -> usize {
        i - Self::isolate_trailing_one(i)
    }
    fn next_sibling(i: usize) -> usize {
        i + Self::isolate_trailing_one(i)
    }
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

    fn with_len(n: usize) -> Self {
        Self {
            tree: vec![T::default(); n + 1],
        }
    }
    fn len(&self) -> usize {
        self.tree.len() - 1
    }
    fn add(&mut self, i: usize, delta: Self::Item) {
        let mut i = i + 1;
        assert!(i < self.tree.len());

        while i < self.tree.len() {
            self.tree[i] += delta;
            i = Self::next_sibling(i);
        }
    }
    fn sum(&self, i: usize) -> Self::Item {
        let mut i = i + 1;
        assert!(i < self.tree.len());
        let mut sum = T::default();
        while i != 0 {
            sum += self.tree[i];
            i = Self::parent(i);
        }
        sum
    }
    fn range_sum(&self, l: usize, r: usize) -> Self::Item {
        self.sum(r)
            - if l == 0 {
                T::default()
            } else {
                self.sum(l - 1)
            }
    }
}

struct UATree<T> {
    tr: Fenwick<T>,
}

impl<T> UATree<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Copy
        + Clone
        + Default
        + AddAssign
        + Neg<Output = T>,
{
    pub fn with_len(n: usize) -> Self {
        Self {
            tr: Fenwick::with_len(n),
        }
    }
    pub fn access(&self, i: usize) -> T {
        self.tr.sum(i)
    }
    pub fn range_update(&mut self, l: usize, r: usize, v: T) {
        assert!(l < self.tr.len());
        assert!(r < self.tr.len());
        assert!(l <= r);
        self.tr.add(l, v);
        if r + 1 < self.tr.len() {
            self.tr.add(r + 1, -v);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.split_whitespace();
    let t: usize = itr.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = itr.next().unwrap().parse().unwrap();
        let u: usize = itr.next().unwrap().parse().unwrap();
        let mut tree: UATree<i64> = UATree::with_len(n);
        for _ in 0..u {
            let l: usize = itr.next().unwrap().parse().unwrap();
            let r: usize = itr.next().unwrap().parse().unwrap();
            let v: i64 = itr.next().unwrap().parse().unwrap();
            tree.range_update(l, r, v);
        }
        let q: usize = itr.next().unwrap().parse().unwrap();
        for _ in 0..q {
            let i: usize = itr.next().unwrap().parse().unwrap();
            println!("{}", tree.access(i));
        }
    }
}
