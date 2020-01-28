// use std::mem;

pub struct List<T> {
  head: Link<T>,
}

// 其实就是 Option 啊
// enum Link {
//   None,
//   Some(Box<Node>),
// }
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
    let new_node = Node {
      elem,
      // next: self.head,
      // next: mem::replace(&mut self.head, None),
      // mem::replace(&mut option, None) 可以用 option.take() 替代
      next: self.head.take(),
    };
    self.head = Some(Box::new(new_node));
  }

  pub fn pop(&mut self) -> Option<T> {
    // 所有权的转移
    // 首先通过 mem::replace，得到了原来的 self.head (T 类型)
    // Some(node)，所有权转移到了 node 中
    // self.head = node.next，node.next 的所有权转移到了 self.head
    // Some(node.elem)，node.elem 所有权转移到了返回值中
    // match mem::replace(&mut self.head, None) {
    // match self.head.take() {
    //   None => None,
    //   Some(node) => {
    //     self.head = node.next;
    //     Some(node.elem)
    //   }
    // }
    // match option { None => None, Some(x) => Some(y) } 可以用 map() 替代
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| &mut node.elem)
  }

  pub fn len(&self) -> usize {
    let mut len = 0;
    let mut cur_link = self.head.as_ref();
    while let Some(boxed_node) = cur_link {
      cur_link = boxed_node.next.as_ref();
      len += 1
    }
    len
  }
}

///////////////

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

///////////////

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<T> List<T> {
  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter {
      next: self.head.as_ref().map::<&Node<T>, _>(|node| &node),
    }
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
      &node.elem
    })
  }
}

///////////////

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    // let mut cur_link = mem::replace(&mut self.head, None);
    let mut cur_link = self.head.take();

    // cur_link 的所有权转移到 boxed_node 中
    while let Some(mut boxed_node) = cur_link {
      // cur_link = mem::replace(&mut boxed_node.next, None);
      cur_link = boxed_node.next.take();
      // boxed_node 在此 scope 结束后被析构
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn basics() {
    let mut list = List::new();

    // Check None list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some Some just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }

  #[test]
  fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
    // list.peek_mut().map(|&mut value| value = 42);
    list.peek_mut().map(|value| *value = 42);

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
  }

  #[test]
  fn len() {
    let mut list = List::new();
    assert_eq!(list.len(), 0);
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.len(), 3);
    list.pop();
    assert_eq!(list.len(), 2);
  }

  #[test]
  fn into_iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
  }
}
