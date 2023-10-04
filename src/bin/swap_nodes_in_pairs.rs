#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }

  fn from(vals: Vec<i32>) -> Option<Box<Self>> {
    let mut head: Option<Box<Self>> = None;
    let mut itr = &mut head;
    for v in vals {
      let temp = Some(Box::new(Self::new(v)));
      if let Some(node) = itr.as_mut() {
        node.next = temp;
        itr = &mut node.next;
      } else {
        head = Some(Box::new(Self::new(v)));
        itr = &mut head;
      }
    }
    head
  }
}

fn main() {
  let head = ListNode::from(vec![1, 2, 3, 4]);
  println!("{:?}", head);
  let new_head = swap_pairs_iter(head);
  println!("{:?}", new_head);
}

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  match head {
    None => None,
    Some(node_ptr) => {
      let mut node = *node_ptr;
      match node.next {
        None => Some(Box::new(node)),
        Some(mut next_ptr) => {
          node.next = swap_pairs(next_ptr.next);
          next_ptr.next = Some(Box::new(node));
          Some(Box::new(*next_ptr))
        }
      }
    }
  }
}

fn swap_pairs_iter(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
  let mut ptr = dummy_head.as_mut().unwrap();
  while let Some(mut node) = ptr.next.take() {
    // Now ptr.next is None.
    let next_node = node.next.take();
    if next_node == None {
      ptr.next = Some(node);
      break;
    } else {
      let mut next_node = next_node.unwrap();
      node.next = next_node.next;
      next_node.next = Some(node);
      ptr.next = Some(next_node);
      ptr = ptr.next.as_mut().unwrap().next.as_mut().unwrap();
    }
  }
  dummy_head.unwrap().next
}
