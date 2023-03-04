use questions::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
  let root = RefCell::new(TreeNode::new(2));
  let left_child = Rc::new(RefCell::new(TreeNode::new(1)));
  let right_child = Rc::new(RefCell::new(TreeNode::new(4)));
  let right_left_child = Rc::new(RefCell::new(TreeNode::new(3)));
  let right_right_child = Rc::new(RefCell::new(TreeNode::new(5)));
  root.borrow_mut().left = Some(Rc::clone(&left_child));
  root.borrow_mut().right = Some(Rc::clone(&right_child));
  right_child.borrow_mut().left = Some(Rc::clone(&right_left_child));
  right_child.borrow_mut().right = Some(Rc::clone(&right_right_child));
  let tree = Some(Rc::new(root));
  dfs(tree);
}

// in-order traversal: left, then middle, then right.
fn dfs(root: Option<Rc<RefCell<TreeNode>>>) {
  if root == None {
    return;
  }
  let root_rc = root.unwrap();
  let mut stack = Vec::new();
  let mut node = Rc::clone(&root_rc);
  stack.push(Rc::clone(&node));
  while !stack.is_empty() {
    let clone = Rc::clone(&node);
    let left = &clone.borrow().left;
    if let Some(left_child) = left {
      stack.push(Rc::clone(left_child));
      node = Rc::clone(left_child);
    } else {
      let x = stack.pop().unwrap();
      println!("{}", x.as_ref().borrow().val);
      let clone = Rc::clone(&x);
      let right = &clone.borrow().right;
      if let Some(right_child) = right {
        stack.push(Rc::clone(right_child));
        node = Rc::clone(right_child);
      }
    }
  }
}
