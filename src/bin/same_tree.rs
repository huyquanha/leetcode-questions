use questions::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {

}

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
  return p == q; // AMAZING!
}