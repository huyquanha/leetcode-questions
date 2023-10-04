use rand::Rng;

fn main() {
  let mut vec = vec![3, -1, 5, 2, 5, 3, 8, 6, 8, 7];
  sort(&mut vec);
  println!("{:?}", vec);
}

fn sort(nums: &mut Vec<i32>) {
  random_shuffle(nums);
  sort_recv(nums, 0, nums.len() - 1);
}

fn sort_recv(nums: &mut Vec<i32>, lo: usize, hi: usize) {
  if lo >= hi {
    return;
  }
  let v = nums[lo];
  let mut lt = lo;
  let mut gt = hi;
  let mut i = lo + 1;
  while i <= gt {
    if nums[i] < v {
      swap(nums, i, lt);
      i += 1;
      lt += 1;
    } else if nums[i] == v {
      i += 1;
    } else {
      swap(nums, i, gt);
      gt -= 1;
    }
  }
  // Now, elements from [lo, lt - 1] are < v, and elements [gt + 1, hi] > v.
  sort_recv(nums, lo, lt - 1);
  sort_recv(nums, gt + 1, hi);
}

fn random_shuffle(nums: &mut Vec<i32>) {
  let mut rng = rand::thread_rng();
  for i in 0..nums.len() - 1 {
    let j = rng.gen_range(i + 1..nums.len());
    swap(nums, i, j);
  }
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
  let tmp = nums[i];
  nums[i] = nums[j];
  nums[j] = tmp;
}