use std::cmp::{max, min};

// https://leetcode.com/problems/median-of-two-sorted-arrays/description/
// Still not correct, wrong at [1, 2], [3, 4, 5, 6]
fn main() {
  // [1, 2, 3] => median is 2
  // [1, 2, 3, 4] => median is 2.5 [(2 + 3) / 2]

  // [1,2] and [3,4,5,6]
  // 1.5 and 4.5 => [2] and [3,4]

  // find median of 1st array (x)
  // find median of 2nd array (y)

  // if x == y => x && y evenly divides the 2 arrays => x && y is the median
  // if x < y => arr1 => [x, ..end] and [start,..y]
  // if x > y => arr1 = [start,..x] and [y,..end]

  // [1,2] and [3] => 1.5 and 3 => [2] and [3]
  // 2 < 3 =>

  // 1, 1, 2, 3, 4, 5
  // [1, 2, 5] and [1, 3, 4] => 2 < 3
  //    - 2 is smaller than half arr1 (3) and half arr2 (4) and the middle => 2 is smaller than 3 elements.
  //       There are 6 in total so 2 cannot be it. we take only [3], same for the other one we take only [2]
  // [2, 5] and [1, 3] => 3.5 and 2 => [2] and [3]

  // [1, 2, 5] and [3, 4] => 2 and 3.5 => [2, 5] and [3] => 3.5 > 3

  // find median for one array m1
  // - if the other array is odd, it has a middle element m2
  //    - if m1 == m2, m1 is the median (no matter if arr1 is odd or even)
  //    - if m1 < m2
  //        - Example: [1, 2, 3] and [2, 3, 5]
  //          - if arr1 is odd, m1 <= (arr1 - 1) / 2 + (arr2 - 1) / 2 + 1 (m2) = (arr1 + arr2) / 2 => m1 could still be it,
  //           but the one less than m1 in arr1 cannot because then it'd be smaller than (arr1 + arr2) / 2 + 1 (impossible)
  //          - Similarly, "5" cannot be it => we cut the array to [2, 3] and [2, 3]
  //        - Else if arr1 is even, m1 is just imaginary. [1, 2] and [2, 3, 5].
  //           - m1 < arr1 / 2 + 1 + (arr2 - 1) / 2 = (arr1 + arr2 + 1) / 2 = 3 => definitely down to [2], and [2, 3]
  //    - id m1 > m2, it's just going to be the reverse of the above
  // - if the other array is even, it has an imaginary element m2
  //   - if m1 == m2, it's the median no matter what
  //   - if m1 < m2
  //       - if arr1 is odd, [1, 2, 3] and [2, 4] => 2 < 3, m1 < (arr1 - 1) / 2 + arr2 / 2= (arr1 + arr2 + 1) / 2 = 2. [2,3] and [2]
  //       - if arr1 is even e.g [-1, 3] and [1, 2, 3, 4] (1 < 2.5) => [-1, 3] and [1, 2, 3]
  // [-1, 3] and [1, 2, 3], what's the story? -1 is less than 2, 3, 3 (-1, 1, 2, 3, 3) can't be it, => [3] and [1, 2]
  // [-1, 3] and [1, 2] => 1 < 1.5, but we can't get rid of either 

  // [1, 3, 6, 9] and [4] compare with 3 and 6. since 3 < 4 < 6 4 must be the chosen one
  // if it's instead 2, because 2 <= 3, 3 is the chosen one
  // if it's instead 7, because 6 <= 7, 6 is the chosen one.
  // [1, 3, 6] and [x]: if x == 3 => 3 is the chosen one
  // if x > 3 => if x > 6 the chosen is is (3 + 6) / 2. Else if x <= 6 then chosen one is (x + 3) / 2
  // if x < 3 => if x >= 1 the chosen one is (x + 3) / 2. Else if x < 1 then chosen one is (1 + 3) / 2

  // [-1, 3] => 1 => 1 < 1.5; [3]
  // [1, 2] => 1.5; 1 < 1.5
  // -1, 1, 2, 3
  //    1 is smaller than 2 elements, so it can be our median? If it's smaller than the lo_median_idx
  // So we successfully handle the edge case when one of the array got down to 1 element.

  let nums1 = vec![1, 2];
  let nums2 = vec![3, 4, 5, 6];
  println!("{}", find_median_sorted_arrays(nums1, nums2));
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  if nums1.is_empty() {
    return get_median(&nums2, 0, nums2.len() - 1).value;
  } else if nums2.is_empty() {
    return get_median(&nums1, 0, nums1.len() - 1).value;
  }
  find_median(&nums1, 0, nums1.len() - 1, &nums2, 0, nums2.len() - 1)
}

fn find_median(
  nums1: &Vec<i32>,
  lo1: usize,
  hi1: usize,
  nums2: &Vec<i32>,
  lo2: usize,
  hi2: usize,
) -> f64 {
  if lo1 == hi1 && lo2 == hi2 {
    return (nums1[lo1] + nums2[lo2]) as f64 / 2.0;
  }
  // If we come here, nums1 and nums2 are not empty, and one of them are not of length 1.
  if lo1 == hi1 {
    return get_median_with_candiate(nums2, lo2, hi2, nums1[lo1]);
  }
  if lo2 == hi2 {
    return get_median_with_candiate(nums1, lo1, hi1, nums2[lo2]);
  }
  // If we come here, both nums1 and nums2 have more than 1 element right now.
  let median1 = get_median(nums1, lo1, hi1);
  let median2 = get_median(nums2, lo2, hi2);
  if median1.value == median2.value {
    return median1.value;
  }
  // Special case if they both have 2 elements, because we can't reduce either of them further.
  // For generality, assume m1 < m2. We can't ignore arr1_lo_median because we can only prove 
  // it's smaller than 2 elements and we can't ignore arr2_hi_median because we can only prove 
  // it's larger than 2 elements. So we need to do this the old fashion way.
  if lo1 + 1 == hi1 && lo2 + 1 == hi2 {
    let mut temp = Vec::with_capacity(4);
    let mut i = lo1;
    let mut j = lo2;
    while i <= hi1 && j <= hi2 {
      if nums1[i] <= nums2[j] {
        temp.push(nums1[i]);
        i += 1;
      } else {
        temp.push(nums2[j]);
        j += 1;
      }
    }
    while i <= hi1 {
      temp.push(nums1[i]);
      i += 1;
    }
    while j <= hi2 {
      temp.push(nums2[j]);
      j += 1;
    }
    return get_median(&temp, 0, 3).value;
  }
  
  let len1 = hi1 - lo1 + 1;
  let len2 = hi2 - lo2 + 1;
  if median1.value < median2.value {
    // If arr1 is odd, arr2 is odd (even sum), new_lo1 = median1.indexes[0], new_hi2 = median2.index[0]
    // if arr1 is odd, arr2 is even (odd sum), new_lo1 = median1.index[0], new_hi2 = median2.index[0]
    // if arr1 is even, arr2 is odd (odd sum), new_lo1 = median1.indexes[1], new_hi2 = median2.index[0]
    // if arr1 is even, arr2 is even (even sum), new_lo1 = median1.index[0], new_hi2 median2.index[1]
    let new_lo1: usize;
    let new_hi2: usize;
    if len1 % 2 != 0 && len2 %2 != 0 {
      new_lo1 = median1.indexes[0];
      new_hi2 = median2.indexes[0];
    } else if len1 % 2 != 0 && len2 % 2 == 0 {
      new_lo1 = median1.indexes[0];
      new_hi2 = median2.indexes[0];
    } else if len1 % 2 == 0 && len2 % 2 != 0 {
      new_lo1 = median1.indexes[1];
      new_hi2 = median2.indexes[0];
    } else {
      new_lo1 = median1.indexes[0];
      new_hi2 = median2.indexes[1];
    }
    // let new_lo1 = if median1.indexes.len() == 1 {
    //   median1.indexes[0]
    // } else {
    //   median1.indexes[1]
    // };
    // let new_hi2 = median2.indexes[0];
    return find_median(&nums1, new_lo1, hi1, &nums2, lo2, new_hi2);
  } else {
    let new_lo2: usize;
    let new_hi1: usize;
    if len2 % 2 != 0 && len1 %2 != 0 {
      new_lo2 = median2.indexes[0];
      new_hi1 = median1.indexes[0];
    } else if len2 % 2 != 0 && len1 % 2 == 0 {
      new_lo2 = median2.indexes[0];
      new_hi1 = median1.indexes[0];
    } else if len2 % 2 == 0 && len1 % 2 != 0 {
      new_lo2 = median2.indexes[1];
      new_hi1 = median1.indexes[0];
    } else {
      new_lo2 = median2.indexes[0];
      new_hi1 = median1.indexes[1];
    }
    // let new_hi1 = median1.indexes[0];
    // let new_lo2 = if median2.indexes.len() == 1 {
    //   median2.indexes[0]
    // } else {
    //   median2.indexes[1]
    // };
    return find_median(&nums1, lo1, new_hi1, &nums2, new_lo2, hi2);
  }
}

struct Median {
  value: f64,
  indexes: Vec<usize>,
}

fn get_median(nums: &Vec<i32>, lo: usize, hi: usize) -> Median {
  let median_indexes = get_median_indexes(lo, hi);
  if median_indexes.len() == 1 {
    return Median {
      value: nums[median_indexes[0]] as f64,
      indexes: median_indexes,
    };
  }
  return Median {
    value: (nums[median_indexes[0]] + nums[median_indexes[1]]) as f64 / 2.0,
    indexes: median_indexes,
  };
}

fn get_median_indexes(lo: usize, hi: usize) -> Vec<usize> {
  if (lo + hi) % 2 == 0 {
    vec![(lo + hi) / 2]
  } else {
    let i = (lo + hi) / 2;
    vec![i, i + 1]
  }
}

fn get_median_with_candiate(
  nums: &Vec<i32>,
  lo: usize,
  hi: usize,
  candidate: i32,
) -> f64 {
  let median_indexes = get_median_indexes(lo, hi);
  if median_indexes.len() == 1 {
    let median_idx = median_indexes[0];
    if candidate == nums[median_idx] {
      return candidate as f64;
    } else if candidate < nums[median_idx] {
      return (max(candidate, nums[median_idx - 1]) + nums[median_idx]) as f64
        / 2.0;
    } else {
      return (min(candidate, nums[median_idx + 1]) + nums[median_idx]) as f64
        / 2.0;
    }
  } else {
    let lo_median_idx = median_indexes[0];
    let hi_median_idx = median_indexes[1];
    if candidate > nums[lo_median_idx] && candidate < nums[hi_median_idx] {
      return candidate as f64;
    } else if candidate <= nums[lo_median_idx] {
      return nums[lo_median_idx] as f64;
    } else {
      return nums[hi_median_idx] as f64;
    }
  }
}
