# 《深入浅出 Rust》Note - 3

- 第四部分 - 线程安全 - 27 ~ 31 章
  - 27. 线程安全
  - 28. 详解 Send 和 Sync
  - 29. 状态共享
  - 30. 管道
  - 31. 第三方并行开发库
- 第五部分 - 实用设施 - 32 ~ 35 章
  - 32. 项目和模块
  - 33. 错误处理
  - 34. FFI
  - 35. 文档和测试

Rust 编译器可以在编译阶段避免所有的数据竞争 (Data Race) 问题。这也是 Rust 的核心竞争力之一。

## 27. 线程安全

### 27.1 什么是线程

略。

### 27.2 启动线程

Rust 使用 `thread::spawn()` 启动一个子线程，使用 `thread.join()` 等待子线程结束。(常规操作)

```rust
use std::thread;
// child 的类型是 JoinHandle<T>,这个 T 是闭包的返回类型
let child = thread::spawn(move || {
  // 子线程的逻辑
});
// 父线程等待子线程结束
let res = child.join();
```

其它函数：

- thread::sleep()
- thread::yield_now() - 放弃当前线程的执行，要求线程调度器执行线程切换
- thread::current()
- thread::park() - 暂停当前线程，进入等待状态。可以在其它线程上执行该线程对象上的 unpark() 方法来重新唤起该线程
- thread::Thread::unpark() - 恢复一个线程的执行

示例：

```rust
use std::thread;
use std::time::Duration;

fn main() {
  let t = thread::Builder::new()
    .name("child1".to_string())
    .spawn(move || {
      println!("enter child thread.");
      thread::park();
      println!("resume child thread");
    })
    .unwrap();
  println!("spawn a thread");
  thread::sleep(Duration::new(5, 0));
  t.thread().unpark();
  t.join();
  println!("child thread finished");
}
```

### 27.3 免数据竞争

Rust 多线程的特别，不像其它语言的多线程，Rust 没有办法在多线程中直接读写普通的共享变量，除非使用 Rust 提供的线程安全相关的设施，从而避免了数据竞争。

### 27.4 Send & Sync

Rust 实现免数据竞争的两个 trait：

- std::marker::Send - 如果类型 T 实现了 Send 类型，那说明这个类型的变量在不同的线程中传递所有权是安全的
- std::marker::Sync - 如果类型 T 实现了 Sync 类型，那说明在不同的线程中使用 `&T` 访问同一个变量是安全的

## 28. 详解 Send 和 Sync

### 28.1 什么是 Send

- 如果一个类型可以安全地从一个线程 move 进入另一个线程，那它就是 Send 类型。
- 内部不包含引用的类型，都是 Send。
- `Mutex<T>` 是 Send，无论如何访问都要 lock，所以所有权在哪个线程不重要。
- ...

`!Send` 表示非 Send 类型。

### 28.2 什么是 Sync

- 基本数字类型肯定是 Sync
- ...

`!Sync` 表示非 Sync 类型。

暂时没懂，先跳过。

### 28.3 自动推理

Send / Sync 是 marker trait，没有方法，只是用来给类型作标记。

标准库中把所有基本类型，以及标准库中定义的类型，都做了合适 的 Send/Sync 标记。

### 28.4 小结

> Rust 语言本身并不知晓 "线程" "并发" 具体是什么，而是抽象出了一些更高级的概念 Send/Sync，用来描述类型在并发环境下的特性。std::thread::spawn 函数就是一个普通函数，编译器没有对它做任何特殊处理。它能保证线程安全的关键是，它对参数有合理的约束条件。这样的设计使得 Rust 在线程安全方面具备非常好的扩展性。

(顶级的抽象能力 -- 不过看了后面，还是使用加锁的老套路...基本上只有加锁才能符合 Send/Sync)

> Rust 的这个设计实际上将开发者分为了两个阵营，一部分是核心库的开发者，一部分是业务逻辑开发者。对于一般的业务开发者来说，完全没有必要写任何 unsafe 代码，更没有必要为自己的自定义类型去实现 Sync / Send，直接依赖编译器的自动推导即可。这个阵营中的程序员可以完全享受编译器和基础库带来的安全性保证，无须投入太多精力去管理细节，极大地减轻脑力负担。

> 而对于核心库的开发者，则必须对 Rust 的这套机制非常了解。比如，他们可能需要设计自己的 "无锁数据类型" "线程池" "管道"等各种并行编程的基础设施。这种时候，就有必要对这些类型使用 unsafe impl Send/Sync 设计合适的接口。这些库本身的内部实现是基于 unsafe 代码做的，它享受不到编译器提供的各种安全检查。相反，这些库本身才是保证业务逻辑代码安全性的关键。

(got! unsafe 是给核心库开发者使用的)

## 29. 状态共享

基本上和其它语言关于数据同步的概念没有太大差别：加锁，互斥，读写锁，原子操作，条件变量，TLS 等。

(复习一下 c++ 的数据竞争与同步)

- Arc：Rc 用于单线程，Arc 用于多线程，Arc 是 Rc 的线程安全版本，A 表示 Atomic。Arc 用来共享引用，不能修改
- Mutex：用 Arc 配合使用用来修改共享变量
- RwLock：读写锁，和 Mutex 类似，但暴露的 API 不一样
- Atomic：原子操作
- 死锁：Rust 无法在编译期检测出死锁，只能开发者自己注意
- Barrier：多个线程在某个点上一起等待，然后再继续执行
- Convar：条件变量
- 全局变量：Rust 允许全局变量，但修改全局变量必须用 unsafe
- 线程局部存储：TLS

## 30. 管道

(莫不是从 Go 里借鉴过来的?)

mpsc: Multi-producers, single-consumer FIFO queue。在不同线程之间建立一个通信管道 (channel)，一边发送消息，一边接收消息。

> Do not communicate by sharing memory; instead，share memory by communicating. -- Effective Go

### 30.1 异步管道

发送端发送完后就马上返回了。管道缓冲区无限大。

示例：

```rust
use std::thread;
use std::sync::mpsc::channel;
fn main() {
  let (tx, rx) = channel();
  thread::spawn(move|| {
    for i in 0..10 { tx.send(i).unwrap();
    }
  });
  while let Ok(r) = rx.recv() {
    println!("received {}", r);
  }
}
```

### 30.2 同步管道

管道缓冲区容里有限，如果缓冲区满了，发送端发送数据时会阻塞，直到接收端从缓冲区接收数据使缓冲区有剩余空间。

示例：

```rust
use std::thread;
use std::sync::mpsc::sync_channel;
fn main() {
  let (tx, rx) = sync_channel(1);
  tx.send(1).unwrap();
  println!("send first");
  thread::spawn(move|| {
    tx.send(2).unwrap();
    println!("send second");
  });
  println!("receive first {}", rx.recv().unwrap());
  println!("receive second {}", rx.recv().unwrap());
}
```

所以管道无非就是一个消费者模型的 FIFO，自己也容易实现，只不过 Go 和 Rust 把这个实现内置到语言里了。

## 31. 第三方并行开发库

简单介绍了一些第三方开发的并行库，先简单了解一下。

- threadpool - 线程池
- scoped-threadpool
- parking_lot - 类似标准库的 Mutex
- crossbeam - 另一套管道的实现
- rayon - 并行迭代器 (比如应用于快排场景)

---

第五部分 - 实用设施 - 32 ~ 35 章

## 32. 项目和模块

cargo / crate / mod，介绍得比较详细，但有些内容可能过时了。

- crate - 项目，crate 之间不能出现循环引用
- mod - 命名空间，mod 之间可以有循环引用

cargo 也支持类似 npm 中的 workspace，将多个 crate 在一个 repo 中管理，共享相同的 cargo.toml 配置。

### 32.3 模块管理

在一个 crate 中创建模块的几种方式

1. 一个文件中创建内嵌模块，直接使用 mod 关键字即可
1. 一个独立的文件就是一个模块，文件名即模块名
1. 一个文件夹也可以创建一个模块。文件夹内部要有一个 mod.rs 文件，这个文件就是这个模块的入口

(看来借鉴了一些 js 的模块管理)

另外，Rust 中用到的模块只能在入口 main.rs 中统一声明，如果没在 main.rs 中声明，那么此模块就不会被编译。

## 33. 错误处理

Rust 将错误分为两大类：

- 不可恢复错误，使用 panic 处理
- 可恢复错误，使用返回值处理，而且一般使用 Result 类型作为返回值

问号运算符，遇到错误提前返回。

新的 Failure 库，可能将来用来替代标准库的 Error trait。

详略，在《the rust programming language》中有记录。

## 34. FFI

FFI (foreign functions interface)，不同语言之间相互调用的接口。Rust 支持与 C 之间的相互调用，即 Rust 可以调用 C 库 (静态库或动态库)，C 也支持直接调用 Rust 生成的库。当然，导出的接口写法跟一般的 Rust 稍微有一些特别，比如用 `#[no_mangle]` 修饰，向编译器表明编译时不要改变函数名称，函数前还要加上 `extern "c"` 修饰等。示例：

```rust
#[no_mangle]
pub extern "C" fn rust_capitalize(s: *mut c_char) {
  unsafe {
    let mut p = s as *mut u8;
    while *p != 0 {
      let ch = char::from(*p);
      if ch.is_ascii() {
        let upper = ch.to_ascii_uppercase();
        *p = upper as u8;
      }
      p = p.offset(1);
    }
  }
}
```

- 静态库：代码直接内联到二进制中，方法可以直接宿主程序中调用。
- 动态库，在宿主程度中需要先用 dlopen() 加载，加载成功之后才可以调用它的方法。

其余详略，需要时再细看。

## 35. 文档和测试

### 35.1 文档

(虽然放在了最后一章，但我觉得还是挺重要的，尝试把 euler 项目中的普通注释改为文档注释并生成文档)

注释：

- 普通注释：`//`, `/*...*/`
- 会生成文档的注释：`//!` `/*!...*/` (用于模块文档), `///`, `/**...*/` (用于函数文档)

示例：

```rust
mod foo {
  //! 这块文档是给 `foo` 模块做的说明

  /// 这块文档是给函数 `f` 做的说明
  fn f() {
    // 这块注释不是文档的一部分
  }
}
```

文档内部支持 markdown 格式 (记得第一次看到这种功能是在 swift 中)，代码用 \`\` 包围，代码块用 \`\`\` 包围，文档中的代码块在测试时也会被当作测试用例执行 (?? wow, nice!)。

如果文档太长，还可以把文档写在单独的 markdown 中，然后在代码中用一个 attribute 指定相应的文档 (wow, cool!)。示例：

```rust
#![feature(external_doc)]

#[doc(include = "external-doc.md")]
pub struct MyAwesomeType;
```

### 35.2 测试

测试相关的在《the rust programming language》中看过了，这里侧重看一下 benchmark 相关的内容。

使用 `#[bench]` 属性进行 benchmark。示例：

```rust
#[cfg(test)]
   mod tests {
       use super::*;
       use self::test::Bencher;

  #[bench]
  fn big_num(b: &mut Bencher) {
    b.iter(|| gcd(12345, 67890))
  }
}
```

(在 euler 项目中试一下)

Done，完结于 2019.12.18，从 2019.12.01 号开始阅读，每天 2 章，共花了 18 天，跟预计时间差不多。
