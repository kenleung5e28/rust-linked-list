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

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| {
      &node.elem
    })
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| {
      &mut node.elem
    })
  }

  pub fn iter(&self) -> Iter<T> {
    Iter { next: self.head.as_deref() }
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

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_deref();
      &node.elem
    })
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

  #[test]
  fn iter() {
    let mut list = List::new();
    list.push(1997);
    list.push(2046);
    list.push(-32768);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&-32768));
    assert_eq!(iter.next(), Some(&2046));
    assert_eq!(iter.next(), Some(&1997));
  }
}