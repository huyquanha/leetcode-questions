use std::collections::BinaryHeap;

fn main() {
  let grid = vec![vec![1, 3, 1], vec![1, 5, 3], vec![4, 3, 2]];
  assert_eq!(10, min_path_memoize(grid));

  let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
  assert_eq!(12, min_path_memoize(grid));
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
  row: usize,
  col: usize,
  cost: i32,
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    // If the weights are equal, continue comparing the rows.
    // If the rows are equal, continue comparing the cols.
    // By default BinaryHeap is a max heap, so we need to reverse all
    // the ordering i.e. the "max" element should actually be the smallest one
    // we want to put on top.
    // Our "max" element should have the smallest weight, and if the weights are
    // equal, we prioritise bigger rows/cols first because it brings us closer to our target.
    other
      .cost
      .cmp(&self.cost)
      .then(self.row.cmp(&other.row))
      .then(self.col.cmp(&other.col))
  }
}

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
  let m = grid.len();
  let n = grid[0].len();
  let mut pq = BinaryHeap::new();
  let mut dist_to = vec![i32::MAX; m * n];
  dist_to[0] = grid[0][0];
  pq.push(State {
    row: 0,
    col: 0,
    cost: dist_to[0],
  });
  while let Some(State { row, col, cost }) = pq.pop() {
    // The moment a grid (row, col) is popped, we know that the cost we have
    // so far is the smallest cost. Suppose there's a smaller cost that can
    // be discovered later. Because that path is later, it's currently >= cost.
    // And since all the paths are positive, it contradicts the assumption.
    if (row == m - 2 && col == n - 1) || (row == m - 1 && col == n - 2) {
      return cost + grid[m - 1][n - 1];
    }

    // It's possible we add multiple states for the same (row, col), and
    // dist_to has already been set to a smaller value. In that case we
    // ignore this cost.
    if dist_to[to_index(row, col, n)] < cost {
      continue;
    }

    for [r, c] in adj(row, col, m, n).into_iter() {
      let idx = to_index(r, c, n);
      if dist_to[idx] > cost + grid[r][c] {
        dist_to[idx] = cost + grid[r][c];
        pq.push(State {
          row: r,
          col: c,
          cost: dist_to[idx],
        });
      }
    }
  }
  panic!("nothing found")
}

fn adj(r: usize, c: usize, m: usize, n: usize) -> Vec<[usize; 2]> {
  let mut adj: Vec<[usize; 2]> = Vec::new();
  if r < m - 1 {
    adj.push([r + 1, c]);
  }
  if c < n - 1 {
    adj.push([r, c + 1]);
  }
  adj
}

fn to_index(r: usize, c: usize, n: usize) -> usize {
  r * n + c
}

fn min_path_sum_topological(grid: Vec<Vec<i32>>) -> i32 {
  let m = grid.len();
  let n = grid[0].len();
  let mut topological: Vec<[usize; 2]> = Vec::new();
  let max_level = m + n - 2;
  let mut level = 0;
  while level <= max_level {
    for i in 0..(level + 1).min(m) {
      let j = level as i32 - i as i32;
      if j >= 0 && j < n as i32 {
        topological.push([i, j as usize]);
      }
    }
    level += 1;
  }
  // for i in 0..m {
  //   for j in 0..n {
  //     topological.push([i, j]);
  //   }
  // }
  // topological.sort_by(|[r1, c1], [r2, c2]| (r1 + c1).cmp(&(r2 + c2)));

  let mut dist_to = vec![i32::MAX; m * n];
  dist_to[0] = grid[0][0];
  for [row, col] in topological {
    let idx = to_index(row, col, n);
    for [r, c] in adj(row, col, m, n).into_iter() {
      let i = to_index(r, c, n);
      if dist_to[i] > dist_to[idx] + grid[r][c] {
        dist_to[i] = dist_to[idx] + grid[r][c];
      }
    }
  }
  dist_to[m * n - 1]
}

fn min_path_memoize(mut grid: Vec<Vec<i32>>) -> i32 {
  let m = grid.len();
  let n = grid[0].len();
  for i in 0..m {
    for j in 0..n {
      grid[i][j] = min(&grid, i, j) + grid[i][j];
    }
  }
  grid[m - 1][n - 1]
}

fn min(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
  if i == 0 && j == 0 {
    return 0;
  }
  if i == 0 {
    return grid[i][j - 1];
  }
  if j == 0 {
    return grid[i - 1][j];
  }
  // If we are here, both i and j are > 0.
  return grid[i - 1][j].min(grid[i][j - 1]);
}