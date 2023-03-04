use std::{cell::RefCell, rc::Rc, collections::VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub fn contruct_tree(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
  if nums.is_empty() {
    return None;
  }
  if nums[0] == None {
    return None;
  }
  let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
  let mut queue = VecDeque::new();
  queue.push_back(Rc::clone(&root));
  let mut i = 1;
  while !queue.is_empty() && i < nums.len() {
    let node = queue.pop_front().unwrap();
    if let Some(num) = nums[i] {
      let left = Rc::new(RefCell::new(TreeNode::new(num)));
      node.borrow_mut().left = Some(Rc::clone(&left));
      queue.push_back(left);
    }
    i += 1;
    if i >= nums.len() {
      break;
    }
    if let Some(num) = nums[i] {
      let right = Rc::new(RefCell::new(TreeNode::new(num)));
      node.borrow_mut().right = Some(Rc::clone(&right));
      queue.push_back(right);
    }
    i += 1;
  }
  Some(root)
}