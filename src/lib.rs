use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::mem;

struct LRUCache {
    a: Vec<(i32, i32)>, // key, value
    m: HashMap<i32, usize>,
    capacity: usize,
    to_remove: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            a: LRUCache::new_vec(capacity as usize),
            m: HashMap::new(),
            capacity: capacity as usize,
            to_remove: 0,
        }
    }

    fn new_vec(capacity: usize) -> Vec<(i32, i32)> {
        // number chosen based on a small number of experiments but obviously highly dependant on the workload
        Vec::with_capacity(max(300usize, 3 * capacity))
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(idx) = self.m.get(&key) {
            let value = self.a[*idx].1;
            self.a.push((key, value));
            self.m.insert(key, self.a.len() - 1);
            self.clean_up();
            value
        } else {
            -1
        }
    }

    fn remove_first(&mut self) {
        for i in self.to_remove..self.a.len() {
            let key = self.a[i].0;
            if let Some(idx) = self.m.get(&key) {
                if i == *idx {
                    self.m.remove(&key);
                    self.to_remove = i + 1;
                    return;
                }
            }
        }
    }

    fn clean_up(&mut self) {
        if self.a.len() + 2 < self.a.capacity() {
            return;
        }

        let mut new_vec = LRUCache::new_vec(self.capacity);
        for (i, (key, value)) in self.a.iter().enumerate() {
            if let Some(idx) = self.m.get(key) {
                if *idx == i {
                    new_vec.push((*key, *value));
                    self.m.insert(*key, new_vec.len() - 1);
                }
            }
        }
        self.to_remove = 0;
        self.a = new_vec;
    }

    fn put(&mut self, key: i32, value: i32) {
        self.a.push((key, value));
        self.m.insert(key, self.a.len() - 1);
        self.clean_up();

        if self.m.len() > self.capacity {
            self.remove_first();
        }
    }
}
