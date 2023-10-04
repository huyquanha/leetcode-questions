#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }

  pub fn from_rev(items: &[i32]) -> Option<Box<Self>> {
    let mut list: Option<Box<Self>> = None;
    for &i in items.iter().rev() {
      if list == None {
        list = Some(Box::new(Self::new(i)));
      } else {
        let mut node = Self::new(i);
        node.next = list;
        list = Some(Box::new(node));
      }
    }
    list
  }

  pub fn from_fwd(items: &[i32]) -> Option<Box<Self>> {
    let mut list: Option<Box<Self>> = None;
    let mut it = &mut list;
    for &i in items {
      let temp = Some(Box::new(Self::new(i)));
      if let Some(node) = it.as_mut() {
        node.next = temp;
        it = &mut node.next;
      } else {
        list = temp;
        it = &mut list;
      }
    }
    list
  }
}
