fn main() {
  let mut vec = vec![3, -1, 5, 2, 5, 3, 8, 6, 8, 7];
  sort(&mut vec);
  println!("{:?}", vec);
}

fn sort(nums: &mut Vec<i32>) {
  let mut aux = nums.clone();
  sort_recv(nums, &mut aux, 0, nums.len() - 1);
}

fn sort_recv(nums: &mut Vec<i32>, aux: &mut Vec<i32>, lo: usize, hi: usize) {
  if lo >= hi {
    return;
  }
  let mid = (lo + hi) / 2;
  sort_recv(aux, nums, lo, mid);
  sort_recv(aux, nums, mid + 1, hi);
  merge(nums, aux, lo, mid, hi);
}

fn merge(nums: &mut Vec<i32>, aux: &mut Vec<i32>, lo: usize, mid: usize, hi: usize) {
  let mut i = lo;
  let mut j = mid + 1;
  let mut k = lo;
  while i <= mid && j <= hi {
    if aux[i] <= aux[j] {
      nums[k] = aux[i];
      i += 1;
    } else {
      nums[k] = aux[j];
      j += 1;
    }
    k += 1;
  }
  while i <= mid {
    nums[k] = aux[i];
    i += 1;
    k += 1;
  }
  while j <= hi {
    nums[k] = aux[j];
    j += 1;
    k += 1;
  }
}
