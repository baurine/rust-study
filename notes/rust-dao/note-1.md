# 《Rust 编程之道》Note - 1

- 第一章 新时代的语言
- 第二章 语言精要
- 第三章 类型系统
- 第四章 内存管理
- 第五章 所有权系统

## 第一章 新时代的语言

从全局的角度概括 Rust 的设计哲学，心智模型。

Rust: 侧重性能、安全、并发

- 内存安全：类型系统、所有权系统、借用和生命周期
- 零成本抽象：泛型、trait
- 实用性：错误处理 (失败、错误和异常)，包管理 cargo

语言架构：

1. 混合范式编程：面向对象，函数式
2. 语义：所有权，move，copy，借用，生命周期，drop
3. 类型系统：泛型，trait，一切皆类型，多态，类型推断
4. 安全内存管理：栈，RAII，堆

开发者掌握 1/2/3 就行，4 是编译器的范畴。

学习 Rust：

- 保持初学者心态 (zan!)
- 先学习概念再动手实践
- 把编译器当朋友

## 第二章 语言精要

从全局的角度概括 Rust 的主要语法。后面还会分章节详述。先让读者对 Rust 有一个大概认识。

### 2.1 Rust 语言的基本构成

- 语言规范
- 编译器
- 核心库: primitive，包括 trait, 原始类型，常见功能型数据结构，宏定义。不包括 os/network/heap/concurrent/io
- 标准库: std，包括核心库所有，另外还有并发，IO，运行时；平台抽象；底层操作接口，错误处理等
- 包管理器: cargo

### 2.2 语句与表达式

单元类型 unit ()，默认返回值。

rust 中有分号和没分号意义是不一样的。

### 2.3 变量与绑定

略

### 2.4 函数与闭包

定义 fn

作用域和生命周期

函数指针：函数为一等公民，可以作为函数参数和返回值

const fn: 常量函数，编译期执行的函数

闭包：匿名函数。闭包可以捕捉外部变量，函数不可以。

### 2.5 流程控制

- if...else
- while, loop, for...in (只有这三种，for 只有 for...in 这一种用法)
- match, if let, while let

### 2.6 基本数据类型

- bool
- 数值
- 字符 char
- 数组 `[T; n]`
- Range 1..5 1..=5
- slice `&[T]` `&mut[T]`
- str / String
- 原生指针：`*const T`, `*mut T`
- never 类型：!

### 2.7 复合类型

Tuple / Struct / Enum

携带参数的枚举类型本质上属于函数指针

```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}
fn main() {
  let x: fn(u8,u8,u8,u8) -> IpAddr = IpAddr::V4;
  let y: fn(String) -> IpAddr = IpAddr::V6;
  let home = IpAddr::V4(127,0,0,0);
}
```

### 2.8 常用集合类型

std::collections，先记住一个 Vec 就行，其它用到再看

### 2.9 智能指针

Rust 中的值默认被分配到栈内存，可以通过 `Box<T>` 将值装箱，分配到堆上。

### 2.10 泛型和 trait

trait 是对类型行为的抽象。类似其它语言的 interface，但不同的是，其它语言的 interface 可以用来表示类型，但 rust 不行。

- 静态分发，零成本抽象，类似 c++ 的模版，编译期生成
- 动态分发，类似多态，运行期查找对应对象的方法并执行

### 2.11 错误处理

使用 `Result<T, E>` 进行处理。

### 2.12 表达式优先级

略

### 2.13 注释与打印

略，在《深入浅出 rust》笔记中有详述。

## 第三章 类型系统

### 3.1 通用概念

数据的类型。

- 编译期进行类型检查 - 静态类型
- 运行时进行类型检查 - 动态类型
- 不允许类型的自动隐式转换，在强制转换前不同类型无法进行计算 - 强类型
- 反之 - 弱类型

所以强弱类型与是否静态无关。

3.1.3 类型系统与多态性

- 参数化多态 (泛型)
- Ad-hoc 多态 (trait)
- 子类型多态 (java 等语方中的多态)

Rust 只支持前两种多态，且一般为静多态，发生在编译期。动多态发生在运行时，比如子类型多态。

### 3.2 Rust 类型系统概述

Rust 是一门强类型且类型安全的静态语言。Rust 中一切皆表达式，表达式皆有值，值皆有类型，所以可以说，Rust 中一切皆类型 (...)

3.2.1 类型大小

- 编译期可确定大小的类型 (Sized Type)
- 动态大小的类型 (DST) - 解决办法，胖指针 (也即引用)，胖指针的小大是固定的
- 零大小类型 (ZST) - 单元类型，单元结构体 ...
- 底类型 - never 类型，将特殊情况纳入了类型系统

`&[u32; 5]` 和 `&mut [u32]`，前者是普通指针，占大小为 8 字节，后者为胖指针，占大小为 16 字节。

3.2.2 类型推导

Rust 支持类型推导 (新式语言都开始支持类型推导了，而老版本的 c++/java 都没有类型推导)，但功能没有 haskell 强大，只能在局部范围内进行类型推导。很多情况下还是需要显式标注类型。

turbofish 操作符 - `::<>`

当 Rust 无法从上下文自动推导时，编译器提示错误，要求显式添加类型标注，如下面这种情况：

```rust
fn main() {
  let x = "1";
  println!("{:?}", x.parse().unwrap());
}
```

`parse()` 方法会报错，因为它是一个泛型方法。

解决办法有两种，一种是显式标注左值类型，一种是在右边使用 trubofish 操作符。

```rust
// 1
let int_x: i32 = x.parse().unwrap();
assert_eq!(int_x, 1)

// 2
assert_eq!(x.parse::<i32>().unwrap(), 1);
```

### 3.3 泛型

Rust 的泛型属于静多态，它是一种编译期多态。在编译期，不管是泛型枚举还是泛型函数或结构体，都会被单例化。(同样，代码也会膨胀吧...确实)。好处是性能好，没有运行时开销。

其余略

### 3.4 深入 trait

trait 是对类型在行为上的约束。有 4 种用法：

1. 接口抽象。接口是对类型行为的统一约束
1. 泛型约束
1. 抽象类型。(trait object?)
1. 标签 trait

3.4.1 接口抽象

同一个 trait 的方法可以同时被多个类型实现，在不同的类型中实现的行为不同。

关联类型。(好像其它语言没见过这个)

详略。

3.4.2 泛型约束

详略，其它笔记中都有详细记录了。

3.4.3 抽象类型

trait 还可以用作抽象类型 (AST)。(复习一下什么是抽象类型，c++/java 中都有抽象类/抽象方法，抽象类不可以直接实例化，只能被子类继承并实例，抽象方法被包含在抽象类中，没有实现，由子类负责具体实现。)

对于抽象类型而言，编译器可能无法确定其确切的功能和所占的空间大小，所以 Rust 目前有两种方法处理抽象类型：

- trait 对象
- impl Trait

trait 对象，其实就是动态分发，在其它笔记中已经有记录，但此书还多用了一些笔墨讲解并不是所有的 trait 都可以作为 trait 对象，是有限制条件的，具体遇到再说。

impl Trait，用于声明函数的返回类型，虽然此书还说可以用于输入参数，但其它资料里没有提及 (用于输入参数时和泛型约束是同一个作用吧)。

```rust
fn can_fly(s: impl Fly+Debug) -> impl Fly {
  if s.fly() {
    println!("{:?} can fly", s);
  } else {
    println!("{:?} can't fly", s);
  }
  s
}
```

3.4.4 标签 trait

Rust 提供了 5 个重要的标签，定义在 std::marker 模块中。

- Sized trait - 标识编译期可确定大小的类型
- Unsized trait - 标识 DST 类型
- Copy trait
- Send trait - 跨线程传递所有权
- Sync trait - 跨线程传递共享 (不可变) 引用

标签 trait 是给编译器用的，有点类似 Java 中的注解，无需实现方法。

### 3.5 类型转换

3.5.1 Deref 解引用

隐式类型转换 / 显式类型转换

Rust 中的隐式类型转换基本上只有自动解引用，通过实现 Deref trait 的 deref() 方法可以自定义解引用的实现。规则：如果一个类型 T 实现了 `Deref<Target=U>`，则该类型 T 的引用在应用的时候会被自动转换为类型 U。

```rust
pub trait Deref {
  type Target: ?Sized;
  fn deref(&self) -> &Self::Target;
}
```

比如 `Rc<T>` 实现了 `Deref<Target<T>>`，所以使用 Rc 时会被自动解引用为 T，可以直接调用 T 的方法。示例：

```rust
use std::rc::Rc;
fn main() {
  let x = Rc::new("hello");
  println!("{:?}", x.chars());  // chars() 是 &str 上的方法，而不是 Rc 上的方法
}
```

当编译器无法检测出是否该使用自动解引用时，需要手动解引用。示例：

```rust
use std:rc::Rc;
fn main() {
  let x = Rc::new("hello");
  let y = x.clone(); // x 为 Rc<&str>
  let z = (*x).clone(); // x 为 &str
}
```

因为 `Rc<&str>` 和 `&str` 都实现了 clone() 方法，所以 `x.clone()` 时就优先使用原始类型的方法，而不是解引用后的类型的方法。

另外，match 时需要手动解引用，自动解引用不是生效。

3.5.2 as 操作符

用于显式类型转换

3.5.3 From/Into trait

略。

### 3.6 当前 trait 系统的不足

略。

### 3.7 总结

略。

## 第四章 内存管理

### 4.1 通过概念

栈 / 堆 / 内存布局：已理解，略。

### 4.2 Rust 中的资源管理

4.2.2 智能指针与 RAII

Rust 的指针大致分三种：

- 引用：`&T`, `&mut T`
- 原生指针：`*const T`, `*mut T`
- 智能指针

引用和原生指针可以用 as 进行转换，比如 `&T as *const T`, `&mut T as *mut T`。

智能指针实际是一种结构体，只不过它的行为类似指针。智能指针是对指针的一层封装，提供了一些额外功能，比如自动释放堆内存。智能指针区别于常规结构体的特性在于，它实现了 Deref 和 Drop 两个 trait。Deref 提供了解引用能力，Drop 提供了自动析构的能力。

String 和 Vec 类型也是一种智能指针。其余略。

4.2.3 内存泄漏与内存安全

Rust 并不能完全防止内存泄漏，内存泄漏并不属于内存安全的范畴。

## 第五章 所有权系统

### 5.1 通用概念

- 值类型：数据直接存储在栈中的数据类型
- 引用类型：数据存储在堆上，栈中只存放指向堆中数据的地址 (指针)

值语义/引用语义

- 值语义：按位复制后，与原始对象无关
- 引用语义：也叫指针语义，一般是将数据存储在堆内存中，通过栈内存的指针来管理堆内存的数据，并且引用语义禁止按位复制。

浅复制，只复制栈上的数据。深复制，栈和堆上的数据一起复制。

Rust 中原生类型都是值语义，这些类型也被称为 POD (Plain Old Data)。

### 5.2 所有权机制

Rust 中分复制 (Copy) 语义和移动 (Move) 语义。复制语义对应值语义，移动语义对应引用语义。(还有借用吧 - 不对，借用也是一种引用)

详略，其它笔记中已记录。

### 5.3 绑定、作用域和生命周期

用 `let` 声明绑定，默认不可变。(语言的逐渐趋势是将变量由默认可变变成默认不可变)。用 mut 显式声明可变。

详略。

5.3.2 绑定的时间属性 - 生命周期

### 5.4 所有权借用

通过引用借用所有权，详略。

### 5.5 生命周期参数

why？为什么 Rust 需要这个，只有理解了 why，才能真正理解它的机理。

> 值的生命周期和词法作用域有关，但是借用可以在各个函数间传递，必然会跨越多个词法作用域，对于这种情况，Rust 的借用检查器无法自动推断借用的合法性，不合法的借用会产生悬垂指针，造成内存不安全。

解决办法就是显式地手动为参数声明生命周期参数。

借用规则一，借用的生命周期不能长于出借方的生命周期。

```rust
fn main() {
  let r;  // 'a start
  {
    let x = 5;  // 'b start
    r = &x;
  } // 'b end
  println!("r: {}", r);
} // 'a end
```

上例中出借方 x 的生命周期是 'b，借用方 r 的生命周期是 'a，'a > 'b，违反借用规则。由于是在函数内的借用，Rust 借用检查器能自动推断出此借用不合法，因此编译不通过。

5.5.1 显式生命周期参数

```rust
&i32;         // 引用
&'a i32;      // 标注生命周期的引用
&'a mut i32;  // 标注生命周期的可变引用
```

**标注生命周期参数并不能改变任何引用的生命周期长短，它只用于编译器的借用检查，**来防止悬垂指针。

函数签名中的生命周期参数

```rust
fn foo<'a>(s: &'a str, t: &'a str) -> &'a str;
```

规则：输出 (借用方) 的生命周期长度必须不长于输入 (出借方) 的生命周期长度。其实和规则一是一样的。

**禁止在没有任何输入参数的情况下返回引用，**因为会明显造成悬垂指针。示例：

```rust
fn return_str<'a>() -> &'a str {
  let mut s = "Rust".to_string();
  for i in 0..3 {
    s.push_str(" Good");
  }
  &s[..]  // "Rust Good Good Good"
}
fn main() {
  let x = return_str();
}
```

此例编译不通过，修正的办法之一是返回 String 类型而不是 &str。

一个正确的示例：

```rust
fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  if s1.len() > s2.len() { s1 } else { s2 }
}
fn main() {
  let s1 = String::from("Rust");
  let s1_r = &s1;
  {
    let s2 = String::from("C");
    let res = the_longest(s1_r, &s2);
    println!("{} is the longest", res);
  }
}
```

结构体定义中的生命周期参数

结构体在含有引用类型成员的时候也需要标注生命周期参数，否则编译不通过。示例：

```rust
struct Foo<'a> {
  part: &'a str,
}
fn main() {
  let words = String::from("Sometimes think, the greatest sorrow than older");
  let first = words.split(',').next().expect("Could not find a ','");
  let f = Foo { part: first };
  assert_eq!("Sometimes think", f.part);
}
```

这里的生命周期参数标记，实际上是和编译器约定了一个规则：结构体实例的生命周期应短于或等于任意一个成员的生命周期。

方法定义中的生命周期参数：比如为上例中的 Foo 实现方法，因为其包含引用类型成员，标注了生命周期参数，所以需要在 impl 关键字之后声明生命周期参数。

```rust
#[derive(Debug)]
struct Foo<'a> {
  part: &'a str,
}
impl<'a> Foo<'a> {
  fn split_fist(s: &'a str) -> &'a str {
    s.split(',').next().expect("Could not find a ','")
  }
  fn new(s: &'a str) -> Self {
    Foo { part: Foo::split_first(s) }
  }
}
fn main() {
  let words = String::from("Sometimes think, the ...");
  println!("{:?}", Foo::new(words.as_str()));
}
```

静态生命周期参数：`'static`。它存活于整个程序运行期间。所有的字符串字面量都有 `'static` 生命周期，类型为 `&'static str`。

```rust
fn main() {
  let x = "hello rust"; // "hello rust" 字面值存储在只读数据段，x 的值是把 "hello rust" 拷贝到栈上
  let y = x; // y 的值是 x 在栈上又进行了一次拷贝
  assert_eq!(x, y);
}
```

5.5.2 省略生命周期参数

三条规则，详略，需要时再细看。

- 每个输入位置上省略的生命周期都将成为一个不同的生命周期参数
- 如果只有一个输入生命周期的位置，则该生命周期都将分配给输出生命周期
- 如果存在多个输入生命周期的位置，但是其中包含着 &self 或 &mut self，则 self 的生命周期都将分配给输出生命周期

  5.5.3 生命周期限定

- `T: 'a` 表示 T 类型中的任何引用都要 "活得" 和 `'a` 一样长
- `T: Trait + 'a` 表示 T 类型必须实现 Trait 这个 trait 且 T 类型中任何引用都要 "活得" 和 `'a` 一样长

  5.5.4 trait 对象的生命周期

暂略。

### 5.6 智能指针与所有权

除了普通的引用 (借用) 类型，Rust 还提供具有移动语义 (引用语义) 的智能指针。两者区别之一就是所有权的不同，智能指针拥有资源的所有权，而普通引用只是对所有权的借用。

```rust
fn main() {
  let x = Box::new("hello");
  let y = x;
  println!("{:?}", x); // error! 所有权转移给了 y
}
```

解引用移动?? (暂不理解，先跳过)

Rust 源码内部使用 box 关键字进行堆内存分配，box 未作为公开 API，内部还包括堆内存分配方法 `exchange_malloc` 和堆内存释放方法 `box_free`。

5.6.1 共享所有权 `Rc<T>` 和 `Weak<T>`

引用计数，多个变量共享多个所有权。`Weak<T>` 弱引用用来解决循环引用导致的内存泄漏。

```rust
fn main() {
  let x = Rc::new(45);
  let y1 = x.clone(); // 增加强引用计数
  let y2 = x.clone(); // 增加强引用计数
  println!("{:?}",  Rc::strong_count(&x));
  let w = Rc::downgrade(&x); // 增加弱引用计数
  println!("{:?}",  Rc::weak_count(&x));
  let y3 = &*x; // 不增加计数
  println!("{}", 100 - *x);
}
```

5.6.2 内部可变性 `Cell<T>` 和 `RefCell<T>`

Rust 中的可变或不可变主要是针对一个变量绑定而言的，比如对于结构体来说，可变或不可变只能对其实例进行设置，而不能设置单个成员的可变性。但是在实际的开发中，某个字段是可变而其它字段不可变的情况确实存在。Rust 提供了 `Cell<T>` 和 `RefCell<T>` 来应对这种情况。它们本质上不属于智能指针，只是可以提供内部可变性的容器。

`Cell<T>` 用来实现字段级可变的情况，且该字段一般是值语义。而 `RefCell<T>` 一般用于引用语义的字段。

Cell 示例：

```rust
use std::cell::Cell;
struct Foo {
  x: u32;
  y: Cell<u32>
}
fn main() {
  let foo = Foo { x: 1, y: Cell::new(3) };
  assert_eq!(1, foo.x);
  assert_eq!(3, foo.y.get());
  foo.y.set(5);
  assert_eq!(5, foo.y.get());
}
```

RefCell 示例：

```rust
use std::cell::RefCell;
fn main() {
  let x = RefCell::new(vec![1, 2, 3, 4]);
  println!("{:?}", x.borrow());
  x.borrow_mut().push(5);
  println!("{:?}", x.borrow());
}
```

`RefCell<T>` 提供了 borrow/borrow_mut 方法，对应 `Cell<T>` 的 get/set 方法。

`Cell<T>` 和 `RefCell<T>` 使用最多的场景就是配合只读引用来使用，比如 `Rc<RefCell<T>>`。

5.6.3 写时复制 `Cow<T>`

真正需要修改时再进行 clone。`Cow<T>` 提供的功能是，以不可变的方式访问借用内容，以及在需要可变借用或所有权的时候再克隆一份数据。`Cow<T>` 实现了 Deref，所以它可以直接调用其包含数据的不可变方法。`Cow<T>` 旨在减少复制操作，提高性能，一般用于读多写少的场景。

`Cow<T>` 的另一个用处是统一实现规范。...

暂未理解，先跳过。

### 5.7 并发安全与所有权

后面会详述。

### 5.8 非词法作用域生命周期 (Non-Lexical Lifetime, NLL)

其它笔记中有记录，略。
