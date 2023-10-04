use questions::index_min_pq::IndexMinPQ;

fn main() {
  let grid = vec![
    vec![1, 3, 1],
    vec![1, 5, 1],
    vec![4, 2, 1]
  ];
  assert_eq!(7, min_path_sum(grid));

  let grid = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
  ];
  assert_eq!(12, min_path_sum(grid));
}

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
  let m = grid.len();
  let n = grid[0].len();
  let mut pq = IndexMinPQ::with_capacity(m * n + 1);
  let mut dist_to = vec![i32::MAX; m * n + 1];
  dist_to[0] = 0;
  pq.insert(0, dist_to[0]);
  while !pq.is_empty() {
    let v = pq.del_min();
    for w in adj(v, m, n).into_iter() {
      relax(&grid, &mut pq, &mut dist_to, v, w, n);
    }
  }
  dist_to[m * n]
}

fn relax(grid: &Vec<Vec<i32>>, pq: &mut IndexMinPQ<i32>, dist_to: &mut Vec<i32>, v: usize, w: usize, n: usize) {
  let [w_r, w_c] = coord(w, n);
  if dist_to[w] > dist_to[v] + grid[w_r][w_c] {
    dist_to[w] = dist_to[v] + grid[w_r][w_c];
    if pq.contains(w) {
      pq.change(w, dist_to[w]);
    } else {
      pq.insert(w, dist_to[w]);
    }
  }
}

fn adj(v: usize, m: usize, n: usize) -> Vec<usize> {
  if v == 0 {
    return vec![1];
  }
  let mut adj = Vec::new();
  let [v_r, v_c] = coord(v, n);
  if v_r < m - 1 {
    // down
    adj.push(v + n);
  }
  if v_c < n - 1 {
    // right
    adj.push(v + 1);
  }
  adj
}

fn coord(v: usize, n: usize) -> [usize; 2] {
  // As v is 1-indexed, we first minus 1 to get 0-indexed value.
  [(v - 1) / n, (v - 1) % n]
}