# 《Rust 编程之道》Note

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

## 第六章 函数、闭包和迭代器

### 6.1 函数

详略，在其它笔记中已有记录。

在函数参数中可以使用模式匹配。

### 6.2 闭包

详略，在其它笔记中已有记录，但此书这块的内容介绍得深入一些，细节先跳过，遇到问题再回来看。

1. 复制语义，以不可变 &T 捕获；如果加上了 move 关键字，则会执行 Copy/Clone，闭包中为 T (我感觉说的不对，从实验来看，复制语义是按 T 捕获，有 move 没 move 都一样，有待进一步确认)
1. 对于移动语义，执行移动语义 move 转移所有权 (并不一定，还要看怎么使用这个变量)
1. 对于可变绑定，如果在闭包中包含对其进行修改操作，则以可变引用 &mut T 捕获

(以上总结我觉得不是很对)

```rust
fn main() {
  let a = 1;
  let b = 2;
  let d = move || a+b; // a 和 b 都是 i32 类型，产生了 Copy/Clone 操作
  let dd = d();
  println!("{}", dd);
  let c = || a+b;  // a 和 b 都是 &i32 类型 (?? 如何确认) ==> 确认了，使用 (*a+*b)，编译器说 a,b 是 integer 类型
  let cc = c();
  println!("{}", cc);

  let s = String::from("hello");
  let ss = || s.len(); // 转移了所有权，s 为 String?? 实际没有转移啊，因为要根据返回值来看，这时返回值是 u32
  println!("{}", ss());
  println!("{}", s); //... 咦，还是可以访问，有点懵圈了，再看看

  let sss = move || s.len();  // 这时才真正转移了所有权，s 的所有权在闭包里
  println!("{}", sss()); // ok，s 的所有权还在闭包 sss 里
  println!("{}", sss()); // ok，s 的所有权还在闭包 sss 里
  println!("{}", s); // wrong，s 不能访问了，因为所有权转移到 sss 中了

  // sss 闭包得到了所有权，但其实是 Fn 型闭包?? 所以可以调用多次??

  let s2 = String::from("rust");
  let s2c = || s2; // 转移了所有权，s2 所有权转称到闭包 s2c 中
  println!("{}", s2c()); // ok，但执行完 s2c() 后，所有权从闭包 s2c 中移出到这一行语句中并被消费掉了
  println!("{}", s2c()); // wrong，s2 的所有权已经不在 s2c 中了
  println!("{}", s2); // wrong
  // 彻底明白了，跟闭包中如何执行还有返回值都有关系
  // s2c 应该是 FnOnce 型闭包，这种闭包只能执行一次

  // 所以是否转移所有权，和是否按 T 捕获，并不一定有关系？

  let s3 = "hello";
  let s3c = move || {println!("{}", s3)}; // copy/clone 了，不用 move 应该也是 copy/clone 了
  s3c();
  s3c();
  println!("{}", s3);
}
```

看 "dive into rust" 22.3 小节笔记吧。捕获方式和闭包自身的类型是独立的。

rust dao 这一部分内容写得有点不够条理...

### 6.3 迭代器

6.3.1 外部迭代器和内部迭代器

外部迭代器也叫主动迭代器，它独立于容器之外...外部迭代器的一个重要特点是，外部可以控制整个遍历过程。一般是用 for 循环来进行控制。

内部迭代器则通过迭代器自身来控制迭代下一个元素，外部无法干预。一般是通过高阶函数来实现。比如 JavaScript 中的 forEach/map，Ruby 中的 each。

Rust 早期实现是内部迭代器，使用较复杂，后来改成了外部迭代器，用 for...in 来进行控制。

迭代器需要实现 Iterator trait。

```rust
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>
}
```

Iterator 还提供了一个叫 `size_hint()` 的方法，用来获取该迭代器还剩余的元素长度，目的是用来优化迭代器。(暂时跳过)

6.3.3 IntoIterator trait

将指定类型转换成迭代器。另外还有对应的 FromIterator trait，进行反操作，将迭代器转换回指定类型。

```rust
pub trait IntoIterator {
  type Item;
  type IntoIter: Iterator<Item=Self::Item>;
  fn into_iter(self) -> Self::IntoIter;
}
```

最常用的集合容器 `Vec<T>` 类型，它实现了 IntoIterator，可以通过 `into_iter()` 方法转换成迭代器。

另外 Rust 还提供不需要转移所有权的迭代器。

- IntoIter，转移所有权，对应 self
- Iter，获得不可变借用，对应 &self
- IterMut，获得可变借用，对应 &mut self

Iter 和 IterMut 迭代器的典型应用是 slice 类型...

Rust 中迭代器不只这三种，String / HashMap 类型还有 Drain 迭代器，可以迭代删除指定范围内的值...

6.3.4 迭代器适配器

Map/Filter/Rev... 不同于 JavaScript/Ruby 中的 filter/map 等方法，Rust 中的迭代器适配器是惰性的，如果没有被下游消费，Map/Filter 这此适配器不会真正地执行 (更像是 Rx 中的 filter/map 等方法)

内部实现也确实跟 Rx 更像，当声明 `.map(|x| x*2)` 时，只是把闭包保存到了内部结构体中，并没有真正的执行这个闭包，当下游进行消费时，才会真正地执行这个闭包。

6.3.5 迭代器消费器

前面说了，Rust 中的迭代器都是惰性的，它们不会自动发生遍历行为，除非调用 next() 方法去消费其中的数据。最直接消费迭代器数据的方法就是 for 循环，它会隐式地调用 next() 方法。

为了便利和性能，Rust 也提供了 for 循环之外的用于消费迭代器中数据的方法，称它们为消费器 (Consumer)。常见的消费器：

- any
- fold (其实就是 reduce)，新版本 Rust 还增加了 sum 消费器
- collect - 将迭代器转换成指定的集合类型，比如 `collect::<Vec<i32>>()`，则迭代器最终会转换成 `Vec<i32>` 这样的数组。因此，它也被称为收集器
- 其它

(JavaScript/Ruby 中没有区分适配器，消费器，统一被称为高阶函数)

6.3.6 自定义迭代器适配器

实现了一个自定义的 step() 迭代器适配器。详略，没太看懂，需要时再看。

## 第七章 结构化编程

### 7.1 面向对象风格编程

7.1.1 结构体

结构体属于代数数据类型之积类型。

Rust 虽然没有像 c++/java 这些语言一样提供 class，只有结构体和 trait，但也一样能实现面向对象风格的编程。此小节举了一个实现示例，详略。

7.1.2 枚举体

枚举体属于代数数据类型之和类型。

此小节将上小节的示例用枚举进行了重构，详略。

7.1.3 析构顺序

略，暂时不需要去了解。

### 7.2 常用设计模式

建造者模式 (builder)，理解。

访问者模式 (visitor)，参考：[设计模式之访问者模式](https://juejin.im/entry/5ab4c3d65188251fc3293550)，大致明白。

RAII 模式 RAII (Resource Acquisition Is Initialization)，参考：

- [维基百科](https://zh.wikipedia.org/wiki/RAII)
- [C++中的 RAII 机制](https://www.jianshu.com/p/b7ffe79498be)

简单地说，就是利用对象离开作用域是会自动调用析构函数释放资源。

在 C++ 中：

> 由于系统的资源不具有自动释放的功能，而 C++ 中的类具有自动调用析构函数的功能。如果把资源用类进行封装起来，对资源操作都封装在类的内部，在析构函数中进行释放资源。当定义的局部变量的生命结束时，它的析构函数就会自动的被调用，如此，就不用程序员显示的去调用释放资源的操作了。

所以并不是什么复杂的东西，就是只要写 C++ 那就是天天在用的东西。

## 第八章 字符串与集合类型

这一章我觉得是写得最好的。

程序中最常用的三大数据结构是：字符串、数组和映射。

字符串是特殊的线性表，是由零个或多个字符组成的有限序列。但字符串和数组、映射的区别在于，字符串是被作为一个整体来关注和使用的；而数组和映射关注最多的是其中的元素及它们之间的关系。所以，数组和映射也被称为集合类型。

Rust 作为一门现代高级语言，为这三大数据结构提供了丰富的操作支持。

### 8.1 字符串

#### 8.1.1 字符编码

字符编码永远是个大头的问题。这本书对 unicode/utf-8 解释得挺清楚。(这个我在关于 python 的笔记中也有详细记录，这里继续作一些补充，但是 Rust 处理字符编码和 Python 中好像不是一样的，待复习一下 Python 中的字符编码)

Unicode 字符集，每个字符对应一个非负整数 (u32，4 字节)，该数字称为码点 (Code Point)。这个仅是 ISO 制定的标准而已，没有规定实现，即没有指定它们如何存储。

utf-8/utf-16/utf-32 则是存储方案。utf-8 最节省空间所以最常用。

utf-8 是以 1 字节为编码单位的可变长编码，它根据一定的规则将码位编码成 1 ～ 4 字节。

具体规则如下所示：

| unicode 码点范围    | utf-8 编码                          |
| ------------------- | ----------------------------------- |
| U+ 0000 ~ U+ 007F   | 0XXXXXXX                            |
| U+ 0080 ~ U+ 07FF   | 110XXXXX 10XXXXXX                   |
| U+ 0800 ~ U+ FFFF   | 1110XXXX 10XXXXXX 10XXXXXX          |
| U+ 10000 ~ U+ 1FFFF | 11110XXX 10XXXXXX 10XXXXXX 10XXXXXX |

按上述规则转换后的示例：

|         | A (英文字符) | 道 (中文字符)  | 😀(emoji)           |
| ------- | ------------ | -------------- | ------------------- |
| unicode | U+0x41       | U+9053         | U+1F600             |
| utf-8   | 0x41         | 0xE9 0x81 0x93 | 0xF0 0x9F 0x98 0x84 |
| byets   | 1            | 3              | 4                   |

将 unicode -> utf-8 称之为编码，将 utf-8 -> unicode 称之为解码 (... really?)。那还有其它字符编码呢，比如 GBK 呢。

因为 Rust 对字符串使用 utf-8 编码，所以在 Rust 中字符串在内存中实际是 u8 的字节数组，即 `Vec<u8>`。

示例：

```rust
use std::str;
fn main() {
  let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
  assert_eq!("道", tao);
  assert_eq!("道", String::from("\u{9053}"));
}
```

`\u` 表示字符的 unicode 码点值。

#### 8.1.2 字符

Rust 使用 char 类型表示单个字符，字面值用单个引号。字符串字面值用双引号。

char 类型使用整数值与 unicode 码点值一一对应。为了能够存储任何 unicode 标量值，rust 规定每个字符都占 4 字节，即 u32。

每个 char 类型的字符都代表一个有效的 u32 类型的整数，但不是每个 u32 类型的整数都能代表一个有效的字符，因此 char::from_u32() 方法的返回值是 Option。

```rust
fn main() {
  let tao = '道';
  let tao_u32 = tao as u32;
  assert_eq!(36947, tao_u32); // 0x9053 = 36947
  println!("U+{:x}", tao_u32); // U+9053
  println!("{}", tao.escape_unicode()) // \u{9053}
  assert_eq!(std::char::from(65), 'A');
  assert_eq!(std::char::from_u32(0x9053), Some('道'));
  assert_eq!(std::char::from_u32(36947), Some('道'));
  assert_eq!(std::char::from_u32(12901010101), None);
}
```

注意，char 类型字符占 4 个字节，但 str/String 类型的字符串中的单个字符并不等于 char 类型，它们是 utf-8 编码，长度不定。(后面还会说)。所以 Rust 中单个字符是原始的 unicode 码，字符串的每个字符是 utf-8 编码。

(印象中 python 在内存中，无论是单个字符，还是字符串中的每个字符，都会转换成 unicode 编码，有待进一步确认)

将 unicode 转换成 utf8：

```rust
fn main() {
  let mut b = [0; 3];
  let tao = '道';
  let tao_str = tao.encode_utf8(&mut b);
  assert_eq!("道", tao_str);
  assert_eq!(3, tao.len_utf8());
}
```

作为基本原生类型，char 提供了一些方便的内建方法：

- is_digit()
- to_digit()
- is_lowcase()
- is_uppercase()
- to_lowcase()
- to_uppercase()
- is_whitespace()
- is_alphabetic()
- is_alphanumeric()
- is_control()
- is_numeric()
- escape_default() - 用于转义 `\t` `\r` `\n` 等特殊字符

#### 8.1.3 字符串分类

Rust 中字符串是 utf-8 编码，每个字符在内存中的长度为 1~4 字节不定。

Rust 中有以下几种字符串：

- str - 表示固定长度字符串
- String - 表示可增长的字符串
- CStr - 表示由 C 分配而被 Rust 借用的字符串，一般用于和 C 交互
- CString - 表示由 Rust 分配且可以传递给 C 函数使用的 C 字符串，同样用于和 C 交互
- OsStr - 表示和操作系统相关的字符串，这是为了兼容 Windows 系统
- OsString - 表示 OsStr 的可变版本，与 Rust 字符串可以相互转换
- Path - 表示路径，定义于 std::path 模块中。Path 包装了 OsStr
- PathBuf - 跟 Path 配对，是 Path 的可变版本。PathBuf 包装了 OsString

str 属于动态大小类型 (DST)，在编译期并不能确定其大小，所以在程序中最常见到的是 str 的切片类型 &str。&str 代表的是不可变的 utf-8 字节序列，创建后无法再为其追加内容或更改其内容。&str 类型的字符串可以存储在任意地方：

- 静态存储区 - 代表是字符串字面量，&'static str，直接存储在已编译的可执行文件中
- 堆分配 - 通过 String 类型字符串取切片生成
- 栈分配 - 比如使用 str::from_utf8() 方法，就可以将栈分配的 [u8; N] 数组转换为一个 &str 字符串

与 &str 类型对应的是 String 类型的字符串。&str 是一个引用类型，而 String 类型的字符串拥有所有权。String 是由标准库提供的可变字符串，本质为一个成员变量是 `Vec<u8>` 类型的结构体，它直接将字符内容存放于堆中。

String 类型由三部分组成：指向堆中字节序列的指针 (`as_ptr()`)、记录堆中字节序列的字节长度 (len())、堆分配的容量 (capacity())。

Rust 提供了多种方法来创建 String：

- `let string: String = String::new();`
- `let string: String = String::from("hello rust");`
- `let string: String = "hello".to_owned();`
- `let string: String = "hello".to_string();`
- `let string: String = 5.to_string();` // "5"

对 String 求切片可以得到 &str: `let str: &str = &string[2..4];`

#### 8.1.4 字符串的两种处理方式

Rust 中的字符串不能使用索引访问其中的字符，因为字符串是 utf-8 字节序列，一个字符由 1~4 字节组成。返回字节还是码点是个问题，不过 Rust 提供了 bytes() 和 chars() 两个方法来分别返回按字节和按字符迭代的迭代器。所以 Rust 中对字符的操作大致分两种：按字节处理和按字符处理。字符串的一些内建方法是默认按节节来处理的，比如 len() 方法返回的是字节长度，而非字符长度。

示例暂略。(不知道怎么敲出书上示例的像一些法语字母的特殊字符...)

虽然字符串不能按索引来随机访问字符，但 Rust 提供了另外两个方法：get() 和 get_mut()，返回值是 Option 类型。

示例暂略。

#### 8.1.5 字符串的修改

追加：

- push(char)
- push_str(&str)
- extend_from_slice(...)

插入：

- insert(pos, char) // pos 应该是按字节计算的位置
- insert_str(pos, &str)

拼接，可以直接用 `+` 和 `+=`。

更新字符串，因为 Rust 不支持通过索引随机访问，所以只能通过迭代器或某些 unsafe 方法，有点麻烦。一个示例，将字符串中偶数位的字符转换成大写。

```rust
fn main() {
  let s = String::from(...);
  let s: String = s.chars().enumerate().map(|(i, c) | {
    if i % 2 == 0 {
      c.to_lowercase().to_string()
    } else {
      c.to_uppercase().to_string()
    }
  }).collect()
}
```

删除字符串：

- remove(pos) // 注意 pos 是字节的索引位置
- pop()
- truncate(n)
- clear()
- drain()

#### 8.1.6 字符串的查找

差不多该有的方法都有了。

- 存在性判断：contains, starts_with, ends_with
- 位置判断：find, rfind
- 分割：split, rsplit ...
- 捕获：matches ...
- 删除区配：trim_matches ...
- 替代区配：replace ...

详略，需要时再细看。

#### 8.1.7 与其它类型互换

将字符串转换成其它类型：parse() 方法。

将其它类型转换为字符串：

- 简单的转换，比如数字，直接调用 to_string() 方法
- 复杂格式化，使用 format! 宏。

format! 的使用，一个示例：`assert_eq!(format!("{:*^12.5}", "HelloRust"), "***Hello***")`

format! 格式化的基本规则：

- 填充字符串宽度：格式为 {:number}，默认用空格填充，也可以显式设置用其它字符填充，比如上例中声明用 `*` 填充空余的空间
- 截取字符串：{:.number}
- 对齐字符串：{:>}, {:^}, {:<}，分别为左对齐、居中、右对齐

`{:*^12.5}` 表示用 `*` 填充空余空间，字符居中，总长度为 12，截取的字符串内容长度为 5。

这些规则可以配合使用。

Rust 还提供了专门针对整数和浮点数的格式化代码。(仅对数字有效)

```rust
assert_eq!(format!("{:>+#15x}", 1234), "         +0x4d2");
assert_eq!(format!("{:>+#015x}", 1234), "+0x0000000004d2");
```

对整数的规则：

- 进制：{:x} 表示十六进制，{:b} 表示二进制
- 符号 `+`，表示强制输出整数的正负符号
- 符号 `#`，用于显示进行的前缀
- 数字 0，表示用数字 0 代替默认的空格用于填充

上例中 `{:>+#015x}` 表示右对齐，显示正负符号，显示进制前缀，用 0 填充，总长度为 15，用十六进制显示。

浮点数，规则又有所不同。`assert_eq!(format!("{:0^12.2}", 1234.5678), "001234.57000")`。小数点后的数字表示要保留的小数位数，而不是总的截取长度了。{:e} 可以转换成科学计数法。

### 8.2 集合类型

Rust 标准库提供的集合类型有以下几种：

- `Vec<T>` - 动态可增长数组
- `VecDeque<T>` - 基于环形缓冲区的 FIFO 双端队列
- `LinkedList<T>` - 双向链表
- `BinaryHeap<T>` - 二叉堆 (最大堆)，可用作优先队列
- `HashMap<K, V>` - 基于哈希表的无序 K-V 映射集
- `BTreeMap<K, V>` - 基于 B 树的有序映射集，按 Key 排序
- `HashSet<T>` - 无序集合
- `BTreeSet<T>` - 基于 B 树的有序集合

最常用的集合类型为 `Vec<T>` 和 `HashMap<K, V>`。

#### 8.2.1 动态可增长数组

Rust 中数组有两种，一种是原生类型 array，类型签名 [T; N]，它拥用固定长度，默认分配在栈上 (可以用 Box 装箱后分配到堆上吗?)。另一种是动态可增长数组 Vec，在堆上分配。

它的创建和操作方法与 String 类似，但它支持用索引随机访问。

- 创建：Vec::new()
- 添加：push(), extend()
- 删除：pop()
- 访问：使用索引，get()
- 查找：contains, stars_with, ends_with ...
- 排序：sort, sort_by, sort_by_key ...

Rust 和排序/比较相关的 trait，偏序/全序 (数学概念，详略)

- PartialEq：代表部分等价关系，定义了 eq 和 ne 两个方法，分别表示 == 和 != 操作
- Eq：代表等价关系，继承自 PartialEq，但其中没有定义任何方法，实际只是用来标记
- PartialOrd：对应于偏序，定义了 partial_cmp, lt, le, gt 和 ge 五个方法
- Ord：对应于全序，定义了 cmp, max 和 min 三个方法

详略。

Vec 的方法也适用于 array。

#### 8.2.2 映射集

增删改查：

- 创建：`HashMap::with_capacity(10)`
- insert()
- remove()
- keys(), values(), contains_key(), get()
- ...

详略。

HashMap 底层实现原理，详略，需要时再细看。

### 8.3 理解容量

略。

### 8.4 小结

略。

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
