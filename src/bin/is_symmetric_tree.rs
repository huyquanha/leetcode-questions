// https://leetcode.com/problems/symmetric-tree/description/
use questions::tree_node::{self, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
  let nums = vec![1, 2, 2, 3, 4, 3, 4].iter().map(|e| Some(*e)).collect();
  let tree = tree_node::contruct_tree(nums);
  println!("{}", is_symmetric_recur(tree));
}

fn is_symmetric_iter(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  let mut stack = VecDeque::new();
  let root = root.unwrap();
  let root = root.as_ref().borrow();
  let tuple = (root.left.as_ref(), root.right.as_ref());
  match tuple {
    (None, None) => {
      return true;
    }
    (None, Some(_)) => {
      return false;
    }
    (Some(_), None) => {
      return false;
    }
    (Some(left), Some(right)) => {
      stack.push_back(Rc::clone(left));
      stack.push_back(Rc::clone(right));
    }
  }
  while !stack.is_empty() {
    let left = stack.pop_front().unwrap();
    let right = stack.pop_front().unwrap();
    let left = left.as_ref().borrow();
    let right = right.as_ref().borrow();
    if left.val != right.val {
      return false;
    }
    let tuple = (left.left.as_ref(), right.right.as_ref());
    let mut result: Option<bool> = None;
    match tuple {
      (None, None) => {}
      (None, Some(_)) => {
        result = Some(false);
      }
      (Some(_), None) => {
        result = Some(false);
      }
      (Some(left), Some(right)) => {
        stack.push_back(Rc::clone(left));
        stack.push_back(Rc::clone(right));
      }
    }
    if let Some(false) = result {
      return false;
    }
    let tuple = (left.right.as_ref(), right.left.as_ref());
    let mut result: Option<bool> = None;
    match tuple {
      (None, None) => {}
      (None, Some(_)) => {
        result = Some(false);
      }
      (Some(_), None) => {
        result = Some(false);
      }
      (Some(left), Some(right)) => {
        stack.push_back(Rc::clone(left));
        stack.push_back(Rc::clone(right));
      }
    }
    if let Some(false) = result {
      return false;
    }
  }
  true
}

fn is_symmetric_recur(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  let root = root.unwrap();
  let root = root.as_ref().borrow();
  let left = &root.left;
  let right = &root.right;
  is_symmetric_branches(left, right)
}

fn is_symmetric_branches(
  left: &Option<Rc<RefCell<TreeNode>>>,
  right: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
  let left = left.as_ref();
  let right = right.as_ref();
  if left == None && right == None {
    return true;
  }
  if left == None && right != None {
    return false;
  }
  if left != None && right == None {
    return false;
  }
  // Here, left and right are both NOT None.
  let left = left.unwrap().as_ref().borrow();
  let left_left = &left.left;
  let left_right = &left.right;

  let right = right.unwrap().as_ref().borrow();
  let right_left = &right.left;
  let right_right = &right.right;

  left.val == right.val
    && is_symmetric_branches(left_left, right_right)
    && is_symmetric_branches(left_right, right_left)
}
