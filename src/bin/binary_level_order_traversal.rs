use questions::tree_node::{self, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
  let tree = tree_node::contruct_tree(vec![
    Some(3),
    Some(9),
    Some(20),
    None,
    None,
    Some(15),
    Some(7),
  ]);
  let result = level_order(tree);
  println!("{:?}", result);
}

struct LevelNode {
  node: Rc<RefCell<TreeNode>>,
  level: usize,
}

impl LevelNode {
  fn new(node: Rc<RefCell<TreeNode>>, level: usize) -> LevelNode {
    LevelNode { node, level }
  }
}

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
  let mut result: Vec<Vec<i32>> = Vec::new();
  if root == None {
    return result;
  }
  let mut queue = VecDeque::new();
  queue.push_back(LevelNode::new(root.unwrap(), 0));
  let mut cur_level = 0;
  let mut cur_vec: Vec<i32> = Vec::new();
  while !queue.is_empty() {
    let level_node = queue.pop_front().unwrap();
    if level_node.level > cur_level {
      cur_level = level_node.level;
      result.push(cur_vec);
      cur_vec = Vec::new();
    }
    let node = level_node.node.as_ref().borrow();
    cur_vec.push(node.val);
    if let Some(left) = &node.left {
      queue.push_back(LevelNode::new(Rc::clone(left), level_node.level + 1));
    }
    if let Some(right) = &node.right {
      queue.push_back(LevelNode::new(Rc::clone(right), level_node.level + 1));
    }
  }
  if !cur_vec.is_empty() {
    result.push(cur_vec);
  }
  result
}
