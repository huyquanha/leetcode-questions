// https://leetcode.com/problems/merge-two-sorted-lists/

use questions::list_node::ListNode;

fn main() {
  let list1 = ListNode::from_fwd(&[1,3,5]);
  let list2 = ListNode::from_fwd(&[2, 3, 5]);
  println!("{:?}", merge_two_lists(list1, list2));
}

fn merge_two_lists(
  list1: Option<Box<ListNode>>,
  list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let mut result: Option<Box<ListNode>> = None;
  let mut itr = &mut result;
  let mut itr1 = &list1;
  let mut itr2 = &list2;
  loop {
    let node1_opt = itr1.as_ref();
    let node2_opt = itr2.as_ref();
    match (node1_opt, node2_opt) {
      (None, None) => {
        return result;
      }
      (Some(node1), None) => {
        itr = populate_node(itr, node1.val);
        itr1 = &node1.next;
      }
      (None, Some(node2)) => {
        itr = populate_node(itr, node2.val);
        itr2 = &node2.next;
      }
      (Some(node1), Some(node2)) => {
        if node1.val <= node2.val {
          itr = populate_node(itr, node1.val);
          itr1 = &node1.next;
        } else {
          itr = populate_node(itr, node2.val);
          itr2 = &node2.next;
        }
      }
    }
  }
}

fn populate_node(mut itr: &mut Option<Box<ListNode>>, val: i32) -> &mut Option<Box<ListNode>> {
  let node = Some(Box::new(ListNode::new(val)));
  if *itr == None {
    *itr = node;
  } else {
    let opt_node = itr.as_mut().unwrap();
    opt_node.next = node;
    itr = &mut opt_node.next;
  }
  itr
}