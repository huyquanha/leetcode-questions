// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

use questions::list_node::ListNode;

fn main() {
  let items = [1, 2, 3, 4, 5];
  let list = ListNode::from_fwd(&items);
  println!("{:?}", list);
  let new_list = remove_nth_from_end(list, 5);
  println!("{:?}", new_list);

  let items = vec![1, 2, 3];
  let list = ListNode::from_rev(&items);
  let new_list = remove_nth_from_end(list, 1);
  println!("{:?}", new_list);
}

fn remove_nth_from_end(
  mut head: Option<Box<ListNode>>,
  n: i32,
) -> Option<Box<ListNode>> {
  // Get the size of the list.
  let mut it = &head;
  let mut sz = 0;
  while let Some(node) = it {
    sz += 1;
    it = &node.next;
  }

  let mut it = &mut head;
  let mut idx: i32 = 0;
  while idx < sz - n {
    if let Some(node) = it {
      it = &mut node.next;
      idx += 1;
    } else {
      panic!("Invalid node");
    }
  }
  *it = it.as_mut().unwrap().next.take();
  if idx == 0 {
    return it.take();
  }
  head
}
