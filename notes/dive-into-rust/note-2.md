# 《深入浅出 Rust》Note - 2

- 第三部分 - 高级抽象 - 21 ~ 26 章
  - 21. 泛型
  - 22. 闭包
  - 23. 动态分派和静态分派
  - 24. 容器与迭代器
  - 25. 生成器
  - 26. 标准库简介

## 21. 泛型

泛型的定义和作用就不用多说了。

### 21.1 数据结构中的泛型

rust 中的泛型支持默认值，而且在等号右边使用时和在左边时不一样。如下所示：

```rust
struct S<T=i32> {
  data: T
}
fn main() {
  let v1 = S { data: 0 }; // let v1:S<i32> = S {data:0};
  let v2 = S::<bool> { data: false }; // let v2:S<bool> = S::<bool> {data: false};
  println!("{} {}", v1.data, v2.data);
}
```

### 21.2 函数中的泛型

```rust
fn compare_option<T>(first: Option<T>, second: Option<T>) -> bool {
  match(first, second) {
    (Some(..), Some(..)) => true,
    (None, None) => true,
    _ => false
  }
}
```

> Rust 没有 C++那种无限制的 ad hoc 式的函数重载功能。现在没有，将来也不会有。主要原因是，这种随意的函数重载对于代码的维护和可读性是一种伤害。通过泛型来实现类似的功能是更好的选择。如果说，不同的参数类型，没有办法用 trait 统一起来，利用一个函数体来统一实现功能，那么它们就没必要共用同一个函数名。

> 我们还有另外一种方案，可以把不同的类型统一起来，那就是 enum。

### 21.3 impl 块中的泛型

impl 时也可以使用泛型。

```rust
impl<T, U> Into<U> for T
  where U: From<T> {
  fn into(self) -> U {
    U::from(self)
  }
}
```

### 21.4 泛型参数约束

Rust 的泛型和 C++ 的 template 非常相似，都具有零抽象成本，在编译期完成，而不是运行时。但也有不同，C++ 是模板类型检查是延迟到到实例化的时候做，没有此 template 没有进行实例，那此 template 即使有错也检测不出来，而 Rust 泛型的类型检查是当场完成的，即使没有实例，如果有错也会编译不通过。

用 trait 对泛型进行类型约束。两种方式，一种是 `T: PartialOrd`，一种是 `where T: PartialOrd`。

```rust
use std::cmp::PartialOrd;
// 第一种写法:在泛型参数后面用冒号约束
fn max<T: PartialOrd>(a: T, b: T) -> T {...}
// 第二种写法,在后面单独用 where 子句指定
fn max<T>(a: T, b: T) -> T
  where T: PartialOrd {...}
```

### 21.5 关联类型

```rust
pub trait Iterator {
  type Item;
  ...
}
```

在 trait 中声明的类型叫做关联类型。关联类型也同样是这个 trait 的 "泛型参数"。只有指定了所有的泛型参数和关联类型，这个 trait 才能真正地具体化。

```rust
use std::iter::Iterator;
use std::fmt::Debug;

fn use_iter<ITEM, ITER>(mut iter: ITER)
    where ITER: Iterator<Item=ITEM>,
    ITEM: Debug {
  while let Some(i) = iter.next() {
    println!("{:?}", i);
  }
}
```

关联类型相比直接使用泛型的一些好处：简化写法，增强可读性...遇到时再回头看。

### 21.6 何时使用关联类型

遇到时再看

### 21.7 泛型特化

先跳过，还没稳定。

## 22. 闭包

Rust 中闭包就是 lambda 表达式，和 JavaScript 中闭包的概念不太一样。

2 个特点：一是可以像函数一样调用，二是可以捕获当前环境中的变量。

### 22.1 变量捕获

闭包的内部原理，生成了一个匿名的 struct，在这个 struct 中存放捕获的外部变量。

三种捕获方式：`&T`, `&mut T`, `T`。

### 22.2 move 关键字

使用 move 关键字时，显式声明使用 `T` 捕获方式，转移所有权。

### 22.3 Fn/FnMut/FnOnce

在 Rust 中，Fn trait 继承自 FnMut，FnMut 继承自 FnOnce。`FnOnce <-- FnMut <-- Fn`。

```rust
pub trait FnOnce<Args> {
  type Output;
  extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
  extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Fn<Args> : FnMut<Args> {
  extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
```

> 对于一个闭包，编译器是如何选择 impl 哪个 trait 呢?答案是，编译器会都尝试一遍，实现能让程序编译通过的那几个。闭包调用 的时候，会尽可能先选择调用 `fn call(&self，args:Args)` 函数，其次尝试选择 `fn call_mut(&mut self，args:Args)` 函数，最后尝试使用 `fn call_once(self，args:Args)` 函数。这些都是编译器自动分析出来的。

重新复习：

```rust
fn main() {
  let v: Vec<i32> = vec![1,2,3];
  let c = || for i in &v { println!("{}", i); };
  c();
  c();
}
```

可以看到，上面这个闭包捕获的环境变量在使用的时候，只需要 `&Vec<i32>` 类型即可，因此它只需要捕获环境变量 v 的引用。因此它能实现 Fn trait。闭包在被调用的时候，执行的是 `fn call(&self)` 函数，所以，调用多次也是没问题的。

我们如果给上面的程序添加 move 关键字，依然可以通过:

```rust
fn main() {
  let v: Vec<i32> = vec![1,2,3];
  let c = move || for i in &v { println!("{}", i); };
  c();
  c();
}
```

可以看到，move 关键字只是影响了环境变量被捕获的方式。第三行，创建闭包的时候，变量 v 被 move 进入了闭包中，闭包中捕获变量包括了一个拥有所有权的 `Vec<i32>`。第四行，闭包调用的时候，根据推断规则，它依然是 Fn 型的闭包，使用的是 `fn call(&self)` 函数，因此闭包变量 c 可以被多次调用。

所以，捕获方式和闭包本身的类型是相互独立的。

- 外部变量捕获方式：`&T`, `&mut T`, `T`，一个闭包里可以捕获多个外部变量，不同的变量可以使用不同的捕获方式。怎么确定闭包中使用了哪种捕抓方式，就看在闭包中怎么用的，比如闭包外的定义是 `let a = 1;`，那么在闭包中如果是按 `a` 用的，那就是按 `T` 捕获；如果是按 `&a` 用的，那就是按 `&T` 捕获 (?? 是吗 - wrong)
- move：改变捕获方式，用了 move 后，捕获方式就变成了 T，外部变量的所有权转移到了闭包内部，一般用于闭包需要传递到函数外部 (escaping closure) 的情况
- 闭包本身的类型及调用方式：Fn/FnMut/FnOnce。Fn/FnMut 类型的闭包可以被调用多次，而 FnOnce 类型的闭包正如其名，只能调用一次。如何确认是哪种类型，只要在闭包中 drop 了任意一个 T，那就只能是 FnOnce 了；否则如果在闭包中通过 &mut T 修改了外部变量，那就是 FnMut 类型，否则就是 Fn 类型。

并不是说按 T 捕获了外部变量，闭包就一定是 FnOnce 类型，主要是看闭包中有没有 drop 了 T。

可以说按 T 捕获了外部变量，闭包不一定是 FnOnce，但如果闭包是 FnOnce 类型，那它一定按 T 捕获了某个外部变量。

但还是有疑问：对于整数这种可复制类型的只读捕获，在闭包中如何体现了 `&T`，明明它用起来是 `T` 的用法啊，用 `*T` 访问它会出错。

```rust
fn main() {
    let mut a = 1;
    let plus_one = || a+1;
    println!("{}", plus_one());
    a += 1;
    println!("{}", plus_one());
}
```

上例编译会出错，提示闭包已经借用了 a，所以后面 mut a 实际已经被冻结了，不能进行 `a+=1;` 的操作，把最后一行注释掉可以通过编译，因为根据 NLL，注释掉最后一行后，闭包的生命周期只到 `a+=1;` 前面一行。

所以，所谓的 `&T` 借用，并不是体现在 `|| a+1;` 中的 a 这个类型上？那到底是体现在哪？是体现在闭包自身类型上？貌似是的：

```rust
// 例子来自 rust dao - 闭包的实现一小节
struct Closure {
    env_var: u32;
}
// ...
fn main() {
    let env_var = 1;
    let mut c = Closure { env_var: env_var };
    c();
    c.call(());
    c.call_mut(());
    c.call_once(());

    let mut c = Closure { env_var: env_var };  // <-- 我以为体现在这
    {
        assert_eq!(3, call_it(&c));  // <-- 实际体现在这？？
    }
}
```

[极客学院 Wiki - 闭包的实现](https://wiki.jikexueyuan.com/project/rust-primer/closure/syntax.html) 这篇解释得不错。对于可复制类型，使用 move 会产生 Copy/Clone 操作；对于引用类型，使用 move 强制转移所有权。

重新梳理总结一下：

(T 捕获和所有权转移不是等价，发生所有权转移和 FnOnce 也不是等价；T 捕获对于复制类型来说，是发生 Copy/Clone，对于引用类型来说，才是发生了所有权转移；发生所有权转移后，还要看这个闭包是否消耗掉了这个 T，只有消耗掉了这个 T，比如 drop 掉或作为返回值返回，才是 FnOnce)

- 不使用 move
  - 无论是复制类型还是引用类型，如果闭包对捕获的外部变量只读，则为 &T 捕获
  - 无论是复制类型还是引用类型，如果闭包对捕获的外部变量进行修改，则为 &mut T 捕获
  - 对于复制类型，不存在所有权转移
  - 对于引用类型，如果在闭包中消耗了外部变量，即获取了它的所有权，则为 T 捕获。且这种情况必须是 FnOnce，且不需要显式地加 move
- 使用 move
  - 对于复制类型，发生 Copy/Clone，T 捕获，但因为复制类型并不存在所有权转移，所以闭包还是 Fn/FnMut
  - 对于引用类型，所有权转移到闭包内，T 捕获，但闭包如果没有 drop 或作为返回值返回掉这个 T，那么闭包是 Fn/FnMut 类型，否则是上面的 FnOnce 类型，这种情况不需要显式地加 move

这么来区分吧，内部和外部。变量和闭包本身。

两个独立对象：变量，闭包。

有没有发生所有权转移，将决定这个变量还能否在闭包外部使用。但不决定这个闭包能调用多少次。只决定变量的使用范围。
- 只对引用类型有效
- 没有转移，闭包外还可以访问这个变量
- 转移，只有闭包能访问这个变量
- 转移分隐式和显式。显式用 move 关键字，隐式，不使用 move，但在闭包内消耗了外部变量，即 drop。

闭包内部对捕获变量的使用，将决定这个闭包能否调用多次。
- 如果闭包对变量是只读，则是 Fn，可任意调用多次。
- 如果闭包对变量进行修改，则是 FnMut，可调用多次。
- 如果闭包对变量进行了 drop，则是 FnOnce，只可调用一次。

### 22.4 闭包与泛型

> 闭包是依靠 trait 来实现的。跟普通 trait 一样，我们不能直接用 Fn FnMut FnOnce 作为变量类型、函数参数、函数返回值。

> 跟普通的 trait 一样，如果我们需要向函数中传递闭包，有下面两种方式：
>
> - 通过泛型的方式。这种方式会为不同的闭包参数类型生成不同版本的函数，实现静态分派。
> - 通过 trait object 的方式。这种方式会将闭包装箱进入堆内存中，向函数传递一个胖指针，实现运行期动态分派。

```rust
// 这里是泛型参数。对于每个不同类型的参数,编译器将会生成不同版本的函数
fn static_dispatch<F>(closure: &F)
  where F: Fn(i32) -> i32 {
  println!("static dispatch {}", closure(42));
}

// 这里是 trait object，`Box<Fn(i32)- >i32>` 也算 trait object
fn dynamic_dispatch(closure: &Fn(i32)->i32) {
  println!("dynamic dispatch {}", closure(42));
}
```

作为返回值时，也有两种写法：

- 静态分发：`fn test() -> impl Fn(i32) -> i32`
- 动态分发，把闭包装箱到堆内存中，使用 `Box<dyn Fn(i32) -> i32>` trait object

### 22.5 闭包与生命周期

先跳过，暂不理解。

## 23. 动态分派和静态分派

```rust
trait Bird {
  fn fly(&self);
}

struct Duck;
struct Swan;

impl Bird for Duck {
  fn fly(&self) { println!("duck duck"); }
}
impl Bird for Swan {
  fn fly(&self) { println!("swan swan");}
}
```

> Rust 可以同时支持静态分派 (static dispatch) 和动态分派 (dynamic dispatch)。
> 所谓静态分派，是指具体调用哪个函数，在编译阶段就确定下来了。Rust 中的静态分派靠泛型以及 impl trait 来完成。对于不同的泛型类型参数，编译器会生成不同版本的函数，在编译阶段就确定好了应该调用哪个函数。
> 所谓动态分派，是指具体调用哪个函数，在执行阶段才能确定。Rust 中的动态分派靠 Trait Object 来完成。Trait Object 本质上是指针，它可以指向不同的类型，指向的具体类型不同，调用的方法也就不同。

trait 是一种 DST 类型，它的大小在编译阶段是不固定的。而 Rust 要求参数大小在编译期能决定的，所以它不能直接用来作为参数类型，及返回值类型。**这是 trait 跟许多语言中的 interface 的一个区别。**

这时我们有两种选择，一种是利用泛型。

```rust
fn test<T: Bird>(arg: T) {
  arg.fly();
}
```

> 这样，test 函数的参数既可以是 Duck 类型，也可以是 Swan 类型。实际上，编译器会根据实际调用参数的类型不同，直接生成不同的函数版本，类似 C++中的 template。

```rust
// 伪代码示意
fn test_Duck(arg: Duck) {
  arg.fly();
}
fn test_Swan(arg: Swan) {
  arg.fly();
}
```

> 所以，通过泛型函数实现的多态，是在编译阶段就已经确定好了调用哪个版本的函数，因此被称为**静态分派**。除了泛型之外，Rust 还 提供了一种 impl Trait 语法，也能实现静态分派。

> 我们还有另外一种办法来实现**多态**，那就是通过指针。虽然 trait 是 DST 类型，但是指向 trait 的指针不是 DST。如果我们把 trait 隐藏到指针 的后面，那它就是一个 trait object，而它是可以作为参数和返回类型的。

```rust
// 根据不同需求，可以用不同的指针类型，如 Box/&/&mut 等
fn test(arg: Box<dyn Bird>) {
  arg.fly();
}
```

> 在这种方式下，test 函数的参数既可以是 `Box<Duck>` 类型，也可以是 `Box<Swan>` 类型，一样实现了**多态**。但在参数类型这里已经将**具体类型**信息抹掉了，我们只知道它可以调用 Bird trait 的方法。而具体调用的是哪个版本的方法，实际上是由这个指针的值来决定的。这就是**动态分派**。

具体内容先略过。我觉得上面的内容解释得已经够详细了。

## 24. 容器与迭代器

### 24.1 容器

Vec, VecDeque, HashMap, BTreeMap... 详细使用略，需要时再看

### 24.2 迭代器

实现了 Iterator trait 的类型。

```rust
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
  ...
}
```

主要方法就是 `next()`，返回值是 `Option<Item>`，一般情况下返回 `Some(Item)`，如果迭代完成，返回 None。

Rust 标准库有一个命名规范，从容器创造出迭代器一般有三种方法:

- `iter()` 创造一个 Item 是 `&T` 类型的迭代器
- `iter_mut()` 创造一个 Item 是 `&mut T` 类型的迭代器
- `into_iter()` 创造一个 Item 是 T 类型的迭代器

迭代器是惰性求值。

对迭代器可以显式地使用 while 加上 next()，也可以用 for 循环，for 循环是专门为迭代器设计的语法糖。

for 循环默认调用的是对象的 `into_iter()` 方法，因此会发生所有权转移。

```rust
let v = vec![1,2,3,4,5,6,7,8,9];
for i in v { // 调用了 v.into_iter()，发生了所有权转移；如果只是想借用，可以改成 `for i in &v {...}`
    println!("{}", i);
}
```

## 25. 生成器

在 Rust 里面，协程 (Coroutine) 是编写高性能异步程序的关键设施，生成器 (Generator) 是协程的基础。

和 JavaScript 中的 generator 还有 Python 中的 generator 一样，使用关键字 yield 实现。调度函数是 resume()。但 resume() 不接受参数，而 js 的 generator 调度函数 next(val) 是接受输入参数的。

定义方式使用闭包的方式，而不是函数的方式。

与迭代器相比，迭代器只是一个 trait，它需要一个 struct 来保存内部状态，而生成器是一个闭包，不需要额外的 struct，内部状态就由生成器自己保存。

### 25.5 协程简介

Rust 的协程设计，核心是 async 和 await 两个关键字，以及 Future 这个 trait:

```rust
pub trait Future {
  type Output;
  fn poll(self: PinMut<Self>, cx: &mut Context) -> Poll<Self::Output>;
  ...
}
```

poll 方法返回 Future 的状态：

```rust
pub enum Poll<T> {
  Ready(T),
  Pending,
}
```

可以把它理解成 JavaScript 中的 Promise，状态相当于 fullfill 和 pending。

因为对 JavaScript 的 Promise/async/await 比较熟悉了，所以这一块理解起来没有什么问题，详略。

## 26. 标准库简介

### 26.1 类型转换

as 用于基本类型的转换。还有其它一些，比如 AsRef/AsMut trait。

```rust
pub trait AsRef<T: ?Sized> {
  fn as_ref(&self) -> &T;
}
pub trait AsMut<T: ?Sized> {
  fn as_mut(&mut self) -> &mut T;
}
```

String 实现了好几种 AsRef:

```rust
impl AsRef<str> for String
impl AsRef<[u8]> for String
impl AsRef<OsStr> for String
impl AsRef<Path> for String
```

Borrow/BorrowMut:

```rust
pub trait Borrow<Borrowed: ?Sized> {
  fn borrow(&self) -> &Borrowed;
}
```

From/Into trait，略。

ToOwned，从一个 `&T` 类型变量创造一个新的 U 类型变量。

ToString/FromStr。

### 26.2 运算符重载

Rust 支持运算符重载。先跳过。

### 26.3 I/O

平台相关的字符串：OsString / OsStr。

文件和路径：PathBuf / Path。

标准输入输出：std::io::stdin() / std::io::stdout()。

进程启动参数，这个和其它语言不一样，启动参数不通过 main() 函数的参数获取，而是通过 std::env::args() 或 std::env::args_os() 获取，进程返回值通过 std::process::exit() 指定。

### 26.4 Any

(啊，原来 Rust 也有 Any...)，估计很少使用，先跳过。
