# Linked List Note

- [Learn Rust With Entirely Too Many Linked Lists](https://github.com/rust-unofficial/too-many-lists)
- [如何在 Rust 中实现链表](https://mp.weixin.qq.com/s/wpqrvyLbt3SySFC5MQWvOA)

从上面两篇文章学习到了如何用 rust 写链表，相比其它语言确实麻烦，主要由所有权导致的。

关键知识点：

- Box
- Option
- 所有权
- std::mem::replace(&mut option, None) 可以用 `option.take()` 替代
- option.map()
- option.as_ref() 可以把 `Option<T>` 变成 `Option<&T>`
- option.as_mut() 可以把 `Option<T>` 变成 `Option<&mut T>`
- push() 操作既可以插入在链表头，也可以插入在链表尾，插入在链表头使操作最方便
- 自定义 drop()
- impl Iterator for IntoIter/Iter/IterMut

代码在 codes/lists 中。
