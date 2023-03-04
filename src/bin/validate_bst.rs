// https://leetcode.com/problems/validate-binary-search-tree/description/

use std::{rc::Rc, cell::RefCell};

use questions::tree_node::TreeNode;

fn main() {

}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  let mut vector = Vec::new();
  visit(&mut vector, &root);
  for i in 1..vector.len() {
    if vector[i] < vector[i - 1] {
      return false;
    }
  }
  true
}

fn visit(vector: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) {
  if *node == None {
    return;
  }
  let node = node.as_ref().unwrap();
  let borrow = node.borrow();
  visit(vector, &borrow.left);
  vector.push(borrow.val);
  visit(vector, &borrow.right);
}