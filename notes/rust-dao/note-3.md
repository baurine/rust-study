# 《Rust 编程之道》Note - 3

- 第九章 构建健壮的程序
- 第十章 模块化编程
- 第十一章 安全并发 (Concurrency)

## 第九章 构建健壮的程序

其它语言为了保证健壮性，处理错误的两大类方式：

- 返回错误值：c
- 异常：c++/java/c# ... 利用栈回退 (Stack Unwind) 或栈回溯 (Stack Backtrace)

### 9.1 通用概念

非正常情况一般分三类：失败 (Failure)、错误 (Error)、异常 (Exception)。

很多支持异常处理的语言，比如 C++、Java、Python 或 Ruby 等，并没有对上述三种情况做出语言级的区分。

现代编程语言 Go 在语言层面上区分了异常 (Panic) 和错误，但是带来了巨大的争议。 在 Go 语言中错误处理是强制性的，开发人员必须显式地处理错误，这就导致 Go 语言代码变得相当冗长，因为每次函数调用都需要 if 语句来判断是否出现错误。

Rust 语言也区分了异常和错误，但相比于 Go 语言，Rust 的错误处理机制就显得非常优雅。

### 9.2 消除失败 (Failure)

Rust 用两种机制来消除失败：

- 强大的类型系统
- 断言

在编译期就能消除函数调用违反 "契约"，即失败的情况。

对于无法在编译期检测出的错误，可以使用断言，即 assert! 系列宏来显式声明继续往下执行需要的条件，条件没达到就 panic，提前快速失败。

### 9.3 分层处理错误 (Error)

Rust 提供了分层式错误处理方案：

- `Option<T>`，用于处理有和无两种情况。
- `Result<T>`，用于处理可以合理解决的问题。比如文件没有找到、权限被拒绝、字 符串解析出错等错误。
- 线程恐慌(Panic)，用于处理无法合理解决的问题。比如为不存在的索引插值，就必须引发线程恐慌。
- 程序中止(Abort)，用于处理会发生灾难性后果的情况，使用 abort 函数可以将进程 正常中止。

#### 9.3.1 可选值 `Option<T>`

Option 可以用来消除空指针问题。

如何处理 Option:

- match
- unwrap 系列方法：expect(), unwrap(), unwrap_or(), unwrap_or_else()，后两者是对 match 的包装

高效处理 Option: map(), map_or(), map_or_else(), and_then()，详略。and_then() 有点像 flat_map，用来防止 Option 嵌套。需要时再细看。

#### 9.3.2 错误处理 `Result<T, E>`

`Result<T, E>` 也是一种枚举体。

```rust
#[must_use]
pub enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

`#[must_use]` 表示必须处理可能的 Err。

高效处理 `Result<T, E>`：Option 的很多方法也适用于 Result，比如 unwrap, map, and_then 等。还有 `?` 语法。此书举了很详细的例子来演示，但先了解一下就行，需要时再细看。

### 9.4 恐慌 (Panic)

对于 Rust 来说，无法合理处理的情况就必须引发恐慌。调用 `panic!()` 手动触发恐慌。

详略。

### 9.5 第三方库

第三方处理错误的库，目前官方推荐的是 failure。详略，需要时再细看。

## 第十章 模块化编程

包管理，crate，mod，可见性。

### 10.1 包管理

cargo，Cargo.toml，详略，其它笔记中已记录。

#### 10.1.2 使用第三方包

在 Cargo.toml 中声明并安装。在 Rust 2015 中使用时还需要在代码中声明 `extern crate xxxx;`，但在 Rust 2018 中不再需要再声明一遍。在 Cargo.toml 中声明的 crate 可以使用 `-`，但在 rust 代码中需要用 `_` 替代，另外 `_rs` 或 `-rs` 也会被强制去掉。

包的使用示例：

- 正则表达式 regex 包
- 惰性静态初始化 `lazy_static` 包：把静态变量的初始化延迟到运行时，使用 `lazy_static!` 宏 (how? 原理?)，详略

在 Cargo.toml 中声明依赖的包时，支持指定从 git 仓库获取，还支持从本地获取。

```toml
[dependencies]
rand= { git =”https ://github.com/rust-lang-nursery/rand” }
hello_world= {path = ”hello world", version = ”0.1.0” }
```

#### 10.1.3 Cargo.toml 文件格式

各个字段的含义，详略，需要时再细看。

#### 10.1.4 自定义 Cargo

即修改 Cargo 的配置文件，可以像 npm 那样通过修改 config 来配置其它的 registry，还可以像 git 那样通过修改 config 配置 alias。默认全局配置在 `~/.cargo/config` 中。

自定义 Cargo 子命令：`cargo --list`，`cargo fmt`

### 10.2 模块系统

- 用 mod 声明模块
- 单个文件同时也是一个默认的模块，文件名就是模块名
- 每个包都拥有一个顶级模块 src/lib.rs 或 src/main.rs，顶级模块名就是该 crate 的名字
- 可以在目录下放置 mod.rs，作用和 js 中目录下的 index.js (在 rust 2018 中 mod.rs 可以省略?)
- 2018 中，如果文件和目录同名，比如 `read_func.rs` 文件和 `read_func` 目录，则目录中的模块都是文件的子模块，即 `read_func` 目录中的模块都是 `read_func` 的子模块。

详略。

### 10.3 从零开始实现一个完整功能包

跟着一块练习了一遍，代码放在 `codes/csv_challenge` 中。

这是一个完整的例子，相当有价值。演示了如何从零开始实现一个有完整功能的 crate，可以学习到如何良好的组织模块，如何进行单元测试/集成测试/benchmark。

特别学习到了 lib.rs 和 main.rs 如何配合工作。

> 这种 main.rs 配合 lib.rs 的形式，是二进制包的最佳实践。

简单地来说，就是 main.rs 只使用从 lib.rs 导出的接口，不和其它任何文件有关联，其实文件最终的接口都通过 lib.rs 重新导出。这样的话，这样 crate 随时可以转换成一个 library。

### 10.4 可见性和私有性

默认私有，加 pub 后可以公开访问。详略，其它笔记已记录。

### 10.5 小结

略。

## 第十一章 安全并发 (Concurrency)

先复习一下 Linux 下的多线程、同步与互斥的相关知识。

(狭义来说，多线程分同步和互斥两种情况，广义来说，可以统称为多线程同步，它包括了互斥)

参考：[Linux 下多线程编程详解](https://www.jianshu.com/p/722c32de3f33)。

- 互斥量
- 条件变量
- 信号量

创建线程：`pthread_create()`，等待线程结束：`pthread_join()`。

线程之间访问相同变量要加锁，使用互斥锁 mutex。`pthread_mutex_lock(&mutex)`，`pthread_mutex_unlock(&mutex)`。

互斥锁一个明显的缺点是它只有两种状态：锁定和非锁定。它无法和其它线程进行联动，只能通过轮循来检测，导致的缺陷：

- 轮询会占用 CPU 资源
- 轮询的时间不好控制，可能导致消费者执行不及时

如果线程之间需要联动，比如线程 A 需要等待线程 B 完成某个操作，这时就还需要在互斥锁的基础上再使用条件变量。

> **条件变量是与互斥量相关联的一种用于多线程之间关于共享数据状态改变的通信机制**。它将解锁和挂起封装成为原子操作。等待一个条件变量时，会解开与该条件变量相关的锁，因此，使用条件变量等待的前提之一就是保证互斥量加锁。线程醒来之后，该互斥量会被自动加锁，所以，在完成相关操作之后需要解锁。

使用条件变量的两个主要 API:

- `pthread_cond_wait(&cond, &mutex)` - 等待条件变量并释放锁，挂起线程，让出 CPU；如果其它线程发出 signal 后，此线程会进行加锁并继续执行
- `pthread_cond_signal(&cond)` - 通知等待条件变量的线程，等待条件变量的某个线程会被唤醒并进行加锁，继续执行

信号量 (信号灯)：

> 线程的信号量与进程间通信中使用的信号量的概念是一样，它是一种特殊的变量，本质上是一个非负的整数计数器，它被用来控制对公共资源的访问。它可以被增加或减少，但对其的关键访问被保证是原子操作。如果一个程序中有多个线程试图改变一个信号量的值，系统将保证所有的操作都将依次进行。

信号量也是用来线程间通信用的，但它和条件变量不一样的地方而于，它可以独立使用，不需要和互斥锁配合使用。

主要 API：

- `sem_init` - 初始化信号量
- `sem_wait(sem_t *sem )` - 被用来阻塞当前线程直到信号量 sem 的值大于 0，解除阻塞后将 sem 的值减 1，表明公共资源经使用后减少
- `sem_post(sem_t *sem)` - 该函数用于以原子操作的方式将信号量的值加 1。当有线程阻塞在这个信号量上时，调用这个函数会使其中的一个线程不在阻塞，选择机制同样是由线程的调度策略决定的
- `sem_destroy` - 销毁信号量

互斥锁/条件变量/信号量区别：

[Linux 线程同步](https://www.jianshu.com/p/86ddb75e6d64)

1. 互斥锁必须在同一个线程 上锁、解锁，信号量则不必。
1. 互斥锁只有两个状态：锁住，解开。
1. 由于信号量有一个与之关联的状态（它的计数值），信号量挂出操作总是被记住。然而当向一个条件变量发送信号时，如果没有线程等待在该条件变量上，那么该信号将丢失。
1. 互斥锁是为了上锁而设计的，条件变量是为了等待而设计的，信号灯即可用于上锁，也可用于等待，因而可能导致更多的开销和更高的复杂性。

上面三个是多线程同步最基本的操作，其它的都是从这三个衍生的。

其它一些名词或概念：

- SpinLock - 自旋锁，一般用于内核，轮循抢锁
- RwLock - 读写锁，解决互斥锁的粒度过大的问题，多个线程只有出现写时才加锁，否则不加锁，提升性能
- 临界区 - windows 中的概念 (接下来再复习一下 windows 中的线程同步)
- 原子操作 - AtomicXxx 系列 API

Windows 中的线程同步：

[C++线程同步的四种方式（Windows）](https://blog.csdn.net/s_lisheng/article/details/74278765)

> 线程之间通信的两个基本问题是互斥和同步。

> - 线程互斥是指对于共享的操作系统资源，在各线程访问时的排它性。当有若干个线程都要使用某一共享资源时，任何时刻最多只允许一个线程去使用，其它要使用该资源的线程必须等待，直到占用资源者释放该资源。
> - 线程同步是指线程之间所具有的一种制约关系，一个线程的执行依赖另一个线程的消息，当它没有得到另一个线程的消息时应等待，直到消息到达时才被唤醒。

> 线程互斥是一种特殊的线程同步 (广义与狭义，广义的线程同步包括互斥)。实际上，互斥和同步对应着线程间通信发生的两种情况：

> - 当有多个线程访问共享资源而不使资源被破坏时；
> - 当一个线程需要将某个任务已经完成的情况通知另外一个或多个线程时。

> 从大的方面讲，线程的同步可分用户模式的线程同步和内核对象的线程同步两大类。

> - 用户模式中线程的同步方法主要有原子访问和临界区等方法。其特点是同步速度特别快，适合于对线程运行速度有严格要求的场合。
> - 内核对象的线程同步则主要由事件、等待定时器、信号量以及互斥量等内核对象构成。由于这种同步机制使用了内核对象，使用时必须将线程从用户模式切换到内核模式，而这种转换一般要耗费近千个 CPU 周期，因此同步速度较慢，但在适用性上却要远优于用户模式的线程同步方式。

> 在 WIN32 中（区别于 Linux，其实也差不多），同步机制主要有以下几种：

> 1. 互斥量 (mutex)
> 1. 事件 (Event)
> 1. 信号量 (semaphore)
> 1. 临界区 (Critical section)

(看似用法一样，但实际有着不同的含义及使用场景。)

上述前三者依次对应 Linux 中的互斥量、条件变量、信号量，但事件不需要和互斥量一起使用，可以独立使用。且这三者都是内核对象，可以跨进程使用。而临界区类似互斥量，但它只是应用层的同步对象，不是内核对象，不能跨进程使用，仅限在一个进程内有效。所以如果不需要跨进程使用，那么在 windows 中优先考虑临界区而不是互斥量。

[四种线程同步（或互斥）方式小结](https://blog.csdn.net/EbowTang/article/details/29905309)

### 11.1 通用概念

并发 (Concurrency) 和并行 (Parallelism)。

> 并发就是**同时应对**多件事情的能力，而并行是**同时执行**多件事情的能力。(这个解释很妙)

软件层面侧重的是并发，而硬件 (如 FPGA) 侧重的是并行。

多进程和多线程，略。每个进程和线程都要消耗系统资源，一台机器不能支持过多的进程和线程数。

事件驱动、异步回调和协程。为了解决 C10K 问题，事件驱动编程应运而生。用一个线程处理多个任务。

事件驱动，最知名的是 linux 的 epoll 技术。事件驱动又称事件轮循。优点：不用做并发的考虑，不需要引入锁，不需要考虑内部调度，只需要依赖于事件，最重要是不会阻塞 (??)。Node.js 是第一个事件驱动编程模型语言。

事件驱动和回调函数虽然解决了 C10K 问题，但写代码时容易陷入回调地狱。新方案的提出：Promise 和 Furture。(原来不单是名字的不同，我之前以为思想是一样的，只不过在不同语言中的叫法不一样。) Promise 站在任务处理者的角度，将异步任务完成或失败的状态标记到 Promise 对象中。Furture 则站在任务调用者的角度，来检测任务是否完成，如果完成则直接获取结果，如果未完成则阻塞直到获取结果 (?? 待进一步深入学习)

为了进一步完善基于事件驱动的编程体验，一种叫**协程**的解决方案浮出水面，可以理解为用户态线程，很轻量，可大量使用。

11.1.3 线程安全

竞争与同步。

竞态条件和临界区。

同步、互斥和原子类型。

- 同步机制消除竞态条件
- 使用互斥和原子类型避免数据竞争

同步：锁、信号量、屏障、条件变量

详略。

### 11.2 多线程并发编程

- 线程管理：std::thread 提供了管理线程的各种函数，比如 join()，notify()...
- 线程同步：std::sync 中提供了锁、channel、条件变量、屏障

#### 11.2.1 线程管理

thread::spawn(), join() ... 详略。

线程本地存储：TLS (Thread Local Storage)

底层同步原语：park/unpark/`yield_now` ... (其它笔记中有记录)

#### 11.2.2 Send 和 Sync

> 从 Rust 提供的线程管理工具来看，并没有什么特殊的地方，和传统语言的线程管理方式非常相似。那么 Rust 是如何做到之前宣称的那样默认线程安全呢？这要归功于 std::marker::Send 和 std::marker::Sync 两个特殊的内置 trait。

- 实现了 Send 的类型，可以安全地在线程间传递所有权。也就是说，可以跨线程移动。
- 实现了 Sync 的类型，可以安全地在线程间传递不可变借用。也就是说，可以跨线程共享。

Rust 通过这两个标签在编译器层面限定了哪些可以在子线程中使用，哪些不行 (比如 Rc 不行，Arc 可以)，避免了其它语言中只能在人为层面进行约束。

本小节列举了很多失败的例子，从这些例子可以看出，在其它语言中有潜在错误的用法在 Rust 中编译都通不过。

#### 11.2.3 使用锁进行线程同步

要修复上一小节中的错误例子，只需要使用支持跨线程安全共享可变变量的容器即可，比如互斥锁 `Mutex<T>`。

例子待补。

跨线程恐慌和错误处理。子线程的 join() 方法返回值是 `Result<T>` 类型，如果在子线程中产生错误，则在 join() 能能拿到它的错误。子线程在获得锁后发生恐慌，称为 "中毒 (posion)"。

读写锁：RwLock。只要线程没拿到写锁，就允许任意数量的线程获得读锁。

#### 11.2.4 屏障 (Barrier) 和条件变量 (Condition Variable)

屏障：通过 wait() 方法在某个点阻塞全部进入临界区的线程，等所有线程都已经 wait() 后再一同继续执行。(有意思)

条件变量和 Linux 中多线程同步的条件变量是一样的，配合互斥量使用。在满足指定条件之前阻塞某一个获得互斥锁的线程。

#### 11.2.5 原子类型

Load/Store, CAS (compare and swap), Swap, Compare-Exchange, Fetch-\*

AtomicBool, AtomicIsize, AtomicPtr, AtomicUsize

内存顺序：Odering::{SeqCst, Relaxed, Release, Acquire, AcqRel}

#### 11.2.6 使用 Channel 进行线程间通信

基于消息通信的并发模型有两种：

- Actor 模型：代表语言 Erlang/Elixir，框架 Akka
- CSP 模型：代表语言 Go

区别：

- Actor 模型中，主角是 actor，actor 之间直接发送、接收消息；CSP 模型中，主角是 channel，并不关心谁发送，谁接收消息
- Actor 模型中，actor 之间是直接通信；CSP 模型中，依靠 channel 通信
- Actor 模弄的耦合度高于 CSP

详略，在其它笔记中已经记录。

#### 11.2.7 内部可变性探究

暂时先跳过。

#### 11.2.8 线程池

暂时先跳过。

#### 11.2.9 使用 Rayon 执行并行任务

暂时先跳过。

#### 11.2.10 使用 Crossbeam

暂时先跳过。

### 11.3 异步并发

#### 11.3.1 生成器

async/await 的支持，首先要有协程的支持。

协程的实现分两种：

- 有栈协程：每个协程自带独立的栈，功能强大，但耗内存，性能稍低
- 无栈协程：基于状态机实现，无独立栈，具体的应用形式为生成器，性能更好，但功能稍弱

Rust 中的协程使用无栈协程。

使用 (类似 ES6)：

```rust
#[feature(generators, generator_trait)]
use std::ops::Generator;
fn main() {
  let mut gen = || {
    yield 1;
    yield 2;
    yield 3;
    return 4;
  }
  unsafe {
    for _ in 0..4 {
      let c = gen.resume();
      println!("{}", c);
    }
  }
}
```

实现原理：

```rust
pub trait Generator {
  type Yield;
  type Return;
  unsafe fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}
```

详略。

生成器与迭代器：如果只关心计算的过程，不关心计算的结果，可以将 Return 设为单元类型，只保留 Yield 类型，也就是 `Generator<Yield=T, Return=()>`，那么生成器就可以化身为迭代器。

用生成器模拟 Future：反过来，如果只关注结果，不关心过程，可以将 Yield 设为单元类型，只保留 Return 的类型，即 `Generator<Yield=(), Return=Result<T, E>>`，生成器就可以化身为 Future。中间过程为 Pending 状态。

(这个抽象妙啊)

#### 11.3.2 Future 并发模式

Rust 对 Future 异步并发模式做了一个完整抽象，包含在第三方库 future-rs 中。该抽象主要包含三个部件：

- Future - 基本的异步计算抽象单元
- Executor - 异步计算调度层
- Task - 异步计算执行层

Future 是一个 trait。

```rust
pub trait Future {
  type Output;
  fn poll(self: Pin<&mut self>, lw: &LocalWaker) -> Poll<Self::Output>;
}

pub enum Poll<T> {
  Ready(T),
  Pending
}
```

poll 方法是 Future 的核心，它是对**轮循**行为的一种抽象。

Executor/Task，详略。

#### 11.3.3 async/await

第三方库 future-rs 经历了三个阶段的迭代，0.1 版本时，开发者可以通过 then 和 `and_then` 方法安排 Future 异步计算的执行顺序 (像 ES6 中的 Promise)，0.3 版本引入了 async/await，使用 `await!` 宏。(本书出版之时，这个功能和写法还没有完全稳定下来，直到 2019 年年末才最终稳定，写法变成 `.await`)

Pin 与 UnPin。

`Pin<T>` 是智能指针。具体原理有点复杂，暂略。引入它的目标是为了解决在 Generator 中引用本地变量的问题。

具体使用，详略。需要时再看吧。

### 11.4 数据并行

SIMD... 先跳过。