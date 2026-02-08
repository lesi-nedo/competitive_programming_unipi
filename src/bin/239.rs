use competitive_programming_unipi::{get_input, get_single_num};
use std::collections::BTreeMap;

struct MultiSet<T> {
    data: BTreeMap<T, usize>,
}

impl<T: Ord> MultiSet<T> {
    fn new() -> Self {
        Self {data: BTreeMap::new()}
    }
    fn add(&mut self, value: T) {
        *self.data.entry(value).or_insert(0) += 1;
    }

    fn remove(&mut self, value: &T) {
        if let Some(count) = self.data.get_mut(value) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.data.remove(value);
            }
        }
    }
    fn get_max(&self) -> Option<(&T, &usize)> {
        self.data.last_key_value()
    }
}

fn max_sliding_window(arr: &[i64], k: usize) -> Vec::<i64> {
     let mut res: Vec::<i64> = Vec::new();
     let mut set: MultiSet<i64> = MultiSet::new();
     for ind in 0..k {
         set.add(arr[ind]);
     }
     if let Some((&val, count)) = set.get_max() {
         res.push(val);
     }
     for ind in 1..arr.len() - k + 1 {
         set.remove(&arr[ind-1]);
         set.add(arr[k+ind-1]);
         if let Some((&val, count)) = set.get_max() {
             res.push(val);
         }
     }
     res
}

fn main () {

     let mut input: Vec<i64> = Vec::new();

     input = get_input(input);
     let k = get_single_num::<usize>();
     let res: Vec<i64> = max_sliding_window(&input, k);
     println!("{:?}", res);

}