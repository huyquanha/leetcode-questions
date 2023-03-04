// https://leetcode.com/problems/add-two-numbers/description/

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
}

fn main() {
  let v1 = vec![9,9,9,9,9,9,9];
  let v2 = vec![9,9,9,9];
  let l1 = construct_list(v1);
  let l2 = construct_list(v2);

  let result = add_two_numbers(l1, l2);
  println!("{:#?}", result);
}

fn construct_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
  let mut list = None;
  let mut current = &mut list;
  for val in nums {
    *current = Some(Box::new(ListNode::new(val)));
    current = &mut current.as_mut().unwrap().next;
  }
  list
}

fn add_two_numbers(
  l1: Option<Box<ListNode>>,
  l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  let mut result = None;
  let mut cur = &mut result;
  let mut carry = 0;
  let mut iter1 = &l1;
  let mut iter2 = &l2;
  // Returns Some(node1,node2) if-and-only-if l1 is Some(node1) and l2 is Some(node2)
  while let Some((b1, b2)) = iter1.as_ref().zip(iter2.as_ref()) {
    let sum = b1.val + b2.val + carry;
    carry = if sum >= 10 { 1 } else { 0 };
    let val = sum % 10;
    *cur = Some(Box::new(ListNode::new(val)));
    cur = &mut cur.as_mut().unwrap().next;
    iter1 = &b1.next;
    iter2 = &b2.next;
  }
  while let Some(b1) = iter1 {
    let sum = b1.val + carry;
    carry = if sum >= 10 { 1 } else { 0 };
    let val = sum % 10;
    *cur = Some(Box::new(ListNode::new(val)));
    cur = &mut cur.as_mut().unwrap().next;
    iter1 = &b1.next;
  }
  while let Some(b2) = iter2 {
    let sum = b2.val + carry;
    carry = if sum >= 10 { 1 } else { 0 };
    let val = sum % 10;
    *cur = Some(Box::new(ListNode::new(val)));
    cur = &mut cur.as_mut().unwrap().next;
    iter2 = &b2.next;
  }
  if carry > 0 {
    *cur = Some(Box::new(ListNode::new(carry)));
  }
  result
}
