pub struct IndexMinPQ<T>
where
  T: Clone + PartialOrd,
{
  // Associate each k (index) with a value.
  vals: Vec<Option<T>>,
  // qp[k] = position of k in the "keys" heap.
  qp: Vec<Option<usize>>,
  // Stores the indexes of vals in a heap order by the values.
  keys: Vec<Option<usize>>,
  n: usize,
}

impl<T> IndexMinPQ<T>
where
  T: Clone + PartialOrd,
{
  pub fn check_heap(&self) -> bool {
    for i in (1..self.n / 2 + 1).rev() {
      let mut j = i * 2;
      if j < self.n && self.less(j + 1, j) {
        j += 1;
      }
      if self.less(j, i) {
        return false;
      }
    }
    true
  }

  pub fn with_capacity(capacity: usize) -> Self {
    IndexMinPQ {
      keys: vec![None; capacity + 1],
      vals: vec![None; capacity],
      qp: vec![None; capacity],
      n: 0,
    }
  }

  pub fn insert(&mut self, k: usize, v: T) {
    self.validate_index(k);
    if self.contains(k) {
      panic!("{k} is already present");
    }
    self.vals[k] = Some(v);
    self.n += 1;
    self.qp[k] = Some(self.n);
    self.keys[self.n] = Some(k);
    self.swim(self.n);
  }

  pub fn change(&mut self, k: usize, v: T) {
    self.validate_index(k);
    if !self.contains(k) {
      panic!("{k} does not exist yet");
    }
    self.vals[k] = Some(v);
    // We don't know how the new value should fit in the order,
    // so we try both swim and sink.
    self.swim(self.qp[k].unwrap());
    self.sink(self.qp[k].unwrap());
  }

  pub fn delete(&mut self, k: usize) {
    self.validate_index(k);
    if !self.contains(k) {
      panic!("{k} does not exist yet");
    }
    let idx = self.qp[k].unwrap();
    self.swap(idx, self.n);
    self.n -= 1;
    self.swim(idx);
    self.sink(idx);
    self.keys[self.n + 1] = None;
    self.vals[k] = None;
    self.qp[k] = None;
  }

  pub fn contains(&self, k: usize) -> bool {
    self.validate_index(k);
    self.vals[k] != None
  }

  pub fn min(&self) -> T {
    if self.is_empty() {
      panic!("PQ is empty");
    }
    self.vals[self.keys[1].unwrap()].clone().unwrap()
  }

  pub fn min_index(&self) -> usize {
    if self.is_empty() {
      panic!("PQ is empty");
    }
    self.keys[1].unwrap()
  }

  pub fn del_min(&mut self) -> usize {
    if self.is_empty() {
      panic!("PQ is empty");
    }
    let k = self.keys[1].unwrap();
    self.swap(1, self.n);
    self.n -= 1;
    self.sink(1);
    self.vals[k] = None;
    self.qp[k] = None;
    self.keys[self.n + 1] = None;
    k
  }

  pub fn is_empty(&self) -> bool {
    self.n == 0
  }

  pub fn size(&self) -> usize {
    self.n
  }

  fn swim(&mut self, mut i: usize) {
    while i > 1 && self.less(i, i / 2) {
      self.swap(i, i / 2);
      i /= 2;
    }
  }

  fn sink(&mut self, mut i: usize) {
    while i * 2 <= self.n {
      let mut j = i * 2;
      if j < self.n && self.less(j + 1, j) {
        j += 1;
      }
      if !self.less(j, i) {
        break;
      }
      self.swap(i, j);
      i = j;
    }
  }

  fn less(&self, i: usize, j: usize) -> bool {
    self.vals[self.keys[i].unwrap()] < self.vals[self.keys[j].unwrap()]
  }

  fn swap(&mut self, i: usize, j: usize) {
    let tmp = self.keys[i];
    self.keys[i] = self.keys[j];
    self.keys[j] = tmp;

    // Also update self.qp
    self.qp[self.keys[i].unwrap()] = Some(i);
    self.qp[self.keys[j].unwrap()] = Some(j);
  }

  fn validate_index(&self, k: usize) {
    if k >= self.vals.len() {
      panic!("Index out of bound")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_insert() {
    let mut pq: IndexMinPQ<char> = IndexMinPQ::with_capacity(5);
    pq.insert(0, 'b');
    pq.insert(1, 'x');
    pq.insert(2, 'd');
    pq.insert(3, 'a');
    assert_eq!(pq.size(), 4);
    // Expects 3 to be first because 'a' is smallest.
    assert_eq!(pq.keys[1].unwrap(), 3);

    assert!(IndexMinPQ::check_heap(&pq));
  }

  #[test]
  #[should_panic(expected = "Index out of bound")]
  fn test_insert_panic() {
    let mut pq: IndexMinPQ<char> = IndexMinPQ::with_capacity(5);
    pq.insert(5, 'a');
  }

  #[test]
  fn test_change() {
    let mut pq: IndexMinPQ<char> = IndexMinPQ::with_capacity(5);
    pq.insert(0, 'b');
    pq.insert(1, 'x');
    pq.insert(2, 'd');
    pq.insert(3, 'a');
    pq.insert(4, 'c');

    // change 0 to e.
    pq.change(0, 'e');
    assert_eq!(pq.size(), 5);

    assert!(IndexMinPQ::check_heap(&pq));
  }

  #[test]
  fn test_contains() {
    let mut pq: IndexMinPQ<char> = IndexMinPQ::with_capacity(5);
    pq.insert(0, 'b');
    pq.insert(1, 'x');
    pq.insert(2, 'd');
    pq.insert(3, 'a');

    assert!(pq.contains(3));
    assert!(!pq.contains(4));
  }

  #[test]
  fn test_delete() {
    let mut pq: IndexMinPQ<char> = IndexMinPQ::with_capacity(5);
    pq.insert(0, 'b');
    pq.insert(1, 'x');
    pq.insert(2, 'd');
    pq.insert(3, 'a');
    pq.insert(4, 'c');

    pq.delete(0);
    assert_eq!(pq.size(), 4);
    assert!(IndexMinPQ::check_heap(&pq));
  }

  #[test]
  fn test_min() {
    let mut pq: IndexMinPQ<char> = IndexMinPQ::with_capacity(5);
    pq.insert(0, 'b');
    pq.insert(1, 'x');
    pq.insert(2, 'd');
    pq.insert(3, 'a');
    pq.insert(4, 'c');

    assert_eq!(pq.min(), 'a');
  }

  #[test]
  fn test_min_index() {
    let mut pq: IndexMinPQ<char> = IndexMinPQ::with_capacity(5);
    pq.insert(0, 'b');
    pq.insert(1, 'x');
    pq.insert(2, 'd');
    pq.insert(3, 'a');
    pq.insert(4, 'c');

    assert_eq!(pq.min_index(), 3);
  }

  #[test]
  fn test_del_min() {
    let mut pq: IndexMinPQ<char> = IndexMinPQ::with_capacity(5);
    pq.insert(0, 'b');
    pq.insert(1, 'x');
    pq.insert(2, 'd');
    pq.insert(3, 'a');
    pq.insert(4, 'c');

    let idx = pq.del_min();

    assert_eq!(idx, 3);
    assert_eq!(pq.size(), 4);
  }
}
