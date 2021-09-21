pub struct List {
  head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem: elem,
      next: self.head.take()
    });
    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<i32> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }
}

impl Drop for List {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
    let mut list = List::new();

    assert_eq!(list.pop(), None);

    list.push(101);
    list.push(202);
    list.push(303);
    assert_eq!(list.pop(), Some(303));
    assert_eq!(list.pop(), Some(202));

    list.push(-999);
    list.push(-1997);
    assert_eq!(list.pop(), Some(-1997));
    assert_eq!(list.pop(), Some(-999));
    assert_eq!(list.pop(), Some(101));
    assert_eq!(list.pop(), None);

  }
}