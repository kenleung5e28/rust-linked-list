pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem: elem,
      next: self.head.take()
    });
    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }
}

impl<T> Drop for List<T> {
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