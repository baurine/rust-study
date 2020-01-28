use std::mem;

pub struct List {
  head: Link,
}

// 其实就是 Option 啊
enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Node {
      elem,
      // next: self.head,
      next: mem::replace(&mut self.head, Link::Empty),
    };
    self.head = Link::More(Box::new(new_node));
  }

  pub fn pop(&mut self) -> Option<i32> {
    // 所有权的转移
    // 首先通过 mem::replace，得到了原来的 self.head (T 类型)
    // Link::More(node)，所有权转移到了 node 中
    // self.head = node.next，node.next 的所有权转移到了 self.head
    // Some(node.elem)，node.elem 所有权转移到了返回值中
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        self.head = node.next;
        Some(node.elem)
      }
    }
  }
}

impl Drop for List {
  fn drop(&mut self) {
    let mut cur_link = mem::replace(&mut self.head, Link::Empty);
    // cur_link 的所有权转移到 boxed_node 中
    while let Link::More(mut boxed_node) = cur_link {
      cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
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

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }
}
