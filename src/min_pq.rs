struct MinPQ<T> where T: PartialOrd + Clone {
  pq: Vec<Option<T>>,
  n: usize
}

impl<T> MinPQ<T> where T: PartialOrd + Clone {
  fn check_is_heap(min_pq: &MinPQ<T>) {
    for k in 1..min_pq.n / 2 + 1 {
      let i = k * 2;
      assert!(min_pq.pq[k].clone().unwrap() <= min_pq.pq[i].clone().unwrap());
      if i < min_pq.n {
        assert!(min_pq.pq[k].clone().unwrap() <= min_pq.pq[i + 1].clone().unwrap());
      }
    }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    MinPQ {
      pq: vec![None; capacity + 1],
      n: 0
    }
  }

  pub fn new() -> Self {
    Self::with_capacity(1)
  }

  pub fn from_vec(keys: Vec<T>) -> Self {
    let n = keys.len();
    let mut pq = vec![None; n + 1];
    for i in 0..n {
      // skip the first position.
      pq[i + 1] = Some(keys[i].clone());
    }
    let mut min_pq = Self {
      pq,
      n
    };
    // heapify it.
    for k in (1..n / 2 + 1).rev() {
      min_pq.sink(k);
    }
    min_pq
  }

  pub fn is_empty(&self) -> bool {
    self.n == 0
  }

  pub fn size(&self) -> usize {
    self.n
  }

  pub fn min(&self) -> T {
    if self.is_empty() {
      panic!("No elements");
    }
    self.pq[1].clone().unwrap()
  }

  pub fn insert(&mut self, v: T) {
    // Double the capacity if needed.
    if self.n == self.pq.len() - 1 {
      self.pq.resize(self.pq.len() * 2, None);
    }
    self.pq[self.n + 1] = Some(v);
    self.n += 1;
    self.swim(self.n);
  }

  pub fn del_min(&mut self) -> T {
    if self.is_empty() {
      panic!("No element");
    }
    let min = self.pq[1].clone().unwrap();
    self.swap(1, self.n);
    self.pq[self.n] = None;
    self.n -= 1;
    self.sink(1);
    if self.n > 0 && self.n == (self.pq.len() - 1) / 4 {
      self.pq.truncate(self.pq.len() / 2);
    }
    min
  }

  fn swim(&mut self, mut k: usize) {
    while k > 1 && self.pq[k] < self.pq[k / 2] {
      self.swap(k, k / 2);
      k /= 2;
    }
  }

  fn sink(&mut self, mut k: usize) {
    while k * 2 <= self.n {
      let mut i = k * 2;
      if i < self.n && self.pq[i + 1] < self.pq[i] {
        i += 1;
      }
      if self.pq[k] <= self.pq[i] {
        break;
      }
      self.swap(i, k);
      k = i;
    }
  }

  fn swap(&mut self, i: usize, j: usize) {
    let tmp = self.pq[i].clone();
    self.pq[i] = self.pq[j].clone();
    self.pq[j] = tmp;
  }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn tets_create_from_vec() {
      let nums = vec![3,6,7,5,2,1,8,9,4];
      let min_pq = MinPQ::from_vec(nums);
      assert_eq!(min_pq.size(), 9);
      MinPQ::check_is_heap(&min_pq);
    }

    #[test]
    fn tets_get_min() {
      let nums = vec![3,6,7,5,2,1,8,9,4];
      let min_pq = MinPQ::from_vec(nums);
      assert_eq!(min_pq.min(), 1);
    }

    #[test]
    fn tets_del_min() {
      let nums = vec![3,6,7,5,2,1,8,9,4];
      let mut min_pq = MinPQ::from_vec(nums);
      let result = min_pq.del_min();
      assert_eq!(result, 1);
      assert_eq!(min_pq.min(), 2);
      assert_eq!(min_pq.size(), 8);
      MinPQ::check_is_heap(&min_pq);
    }

    #[test]
    fn tets_insert() {
      let nums = vec![3,6,7,5,2,1,8,9,4];
      let mut min_pq = MinPQ::from_vec(nums);
      min_pq.insert(0);
      assert_eq!(min_pq.min(), 0);
      assert_eq!(min_pq.size(), 10);
      MinPQ::check_is_heap(&min_pq);
    }
}