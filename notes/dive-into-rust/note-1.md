# 《深入浅出 Rust》Note

## 1. 概述

### 1.4 prelude

- crate : 理解成项目，一个完整的编译单元，可以编译成 lib 或可执行文件
- mod: 理解成 namespace

极简标准库 std，一些常用的依赖，rust 放到了 std::prelude 模块中，每个项目都会自动导入 `std::prelude::*`。

## 2. 变量和类型

rust 变量先声明后使用，使用前必须初始化。(有没有 lateinit?)

绑定

模式解构

mut x 是一个组合

```rust
let (mut a, mut b) = (1, 2);
let Point { x: ref a,  y: ref b} = p;
```

变量庶蔽，类型推导

用 type 声明类型别名

- static - 静态变量，全局，如果要修改，必须放到 unsafe 块中
- const - 常量，不可用 mut 修饰，编译时可能被内联掉

### 2.2 基本数据类型

- bool
- char: 存储单个 unicode 字符，4 个字节，如果想用一个字节存储字符，前面加上 b 声明。比如 `let y:u8 = b'A'; let s:&[u8;5] = b"hello";`
- 整数类型：i8/i16/i32/i64/i128/isize, u8/u16/u32/u64/u128/usize，其中 isize/usize 是指针大小，长度和系统相关，一般是 4 字节 (32 位系统) 或 8 字节 (64 位系统)

#### 2.2.4 整数溢出

c 语言对这个的处理比较随意，不会出错，取决于开发者。rust 倾向于预防 bug。debug 模式下溢出会 panic，release 下直接截断。对安全性较高的可以使用 `check_*`, `saturating_*`, `wrapping_*` 系列函数。

#### 2.2.5 浮点类型

f32/f64 Nan, Infinite ...

#### 2.2.6 指针类型

`Box<T>`, `&T`, `&mut T`, `*const T`, `*mut T` (没有 `*T` 的写法)

- `Box<T>` - 有所有权的指针
- `&T` - 无所有权，借用，只读
- `&mut T` - 无所有权，借用，可写
- `*const T` - 只读裸指针，无生命周期
- `*mut T` - 可写裸指针，无生命周期

智能指针：

- `Rc<T>`
- `Arc<T>`
- `Cow<'a, T>`

#### 2.2.7 类型转换

as

### 2.3 复合数据类型

- tuple - (a, b, c)
- 空元组 - () ，单元类型
- 结构体：struct，三种
  - 普通 struct Point { x:i32, y: i32 }
  - tuple struct: struct Inches(i32); 用来实现类型别名 alias, struct Color(i32, i32, i32)
  - 空 struct，只用来实现 trait
- enum: 也有几种表示形式。代数类型系统。

#### 2.3.5 - 类型递归定义

略。

### 4.2 发散函数 (Diverging functions)

如果一个函数不能正常返回，那么它的返回类型是 `!`，称之为发散函数。比如：

```rust
fn diverges() -> ! {
  panic!("This function never returns!");
}
```

发散函数的最大特点，它可以被转换成任意一个类型。

(哦，这解答了我一直存在的一个疑问：`let guess: u32 = guess.trim().parse().expect("failed");`，即 `expect()` 不是有可能 panic 吗？那怎么能和 `guess: u32` 类型匹配呢?)

为什么需要这样的一种返回类型，看示例：

```rust
let p = if x {
  panic!("error");
} else {
  100
}
```

因为对于 if...else 分支来说，两个分支的类型必须相同，所以编译器规则 `!` 可以和任意类型相容，这样才能通过编译。

`continue;`, `break;` 的返回类型也是 never type。

## 5. trait

rust 中的 trait 和其它语言的 interface(java)/protocol(swift) 有些类似，但又有很大的不一样。

在 java/swift 中，interface 可以作为变量类型，可以作为参数类型，可以作为返回值类型，简而言之，就是 interface 可以和 class 一样作为类型使用，但在 rust 中，trait 并不是类型，所以上面的用法，它统统不能使用。(解答了我一些之前的疑惑，所以，rust 不用 interface 这个关键字而用 trait 是有道理的。)

在其它语言中，假设 Shape 是 interface，Circle 是它的实现类，那么下面的类似代码是合理的：

```rust
let x: Shape = Circle::new();
fn use_shape(arg: Shape) {...}
fn ret_shape() -> Shape {...}
```

但在 rust 中，假设 Shape 是 trait，Circle 是 struct，且 impl Shape for Cicle，上面的用法都是错误的，因为：

- trait 不能做局部变量的类型
- trait 不能直接做参数的类型
- trait 不能直接做返回值的类型

那要想在 rust 实现上面的这些功能怎么办，使用泛型。比如 `fn use_shape<T: Shape>(arg: T) {...}`。

### 5.1 成员方法

成员方法的第一个参数其实是省略写法。

```rust
// 完整写法
trait T {
  fn method1(self: Self);
  fn method2(self: &Self);
  fn method3(self: &mut Self);
}
// 省略写法
trait {
  fn method1(self);
  fn method2(&self);
  fn method3(&mut self);
}
```

大写的 Self 表示类型，它指代目标 struct，小写的 self 是相应的实例变量。

### 5.2 静态方法

第一个参数不是 self 的方法。

### 5.3 扩展方法

类似 c#/swift/kotlin 这些新式语言，可以给任意存在的类型声明新的成员方法，比如如下所示：

```rust
trait Double {
  fn double(&self) -> Self;
}
impl Double for i32 {
  fn double(&self) -> i32 { *self * 2 }
}
...
```

但也有限制，既所谓的孤儿规则：impl 块要么与 trait 的声明在同一个 crate 中，要么与类型的声明在同一个 crate 中。如果 trait 和 类型都来自外部，那编译器不允许你为这个类型 impl 该 trait。

---

## 11. 所有权和移动语义

### 11.2 移动语义

一个变量可以把它拥有的值转移给另一个变量，称为 "所有权转移"，赋值语句、函数调用、函数返回等，都有可能导致所有权转移。

```rust
fn create() -> String {
  let s = String::from("hello");
  return s;  // 所有权转移，从函数内部移动到外部
}
fn consume(s: String) { // 所有权转移，从函数外部移动到内部
  println!("{}", s);
  // 没有继续转移所有权，s 将会被销毁
}
fn main() {
  let s = create();
  consume(s);

  // error!
  // println!("{}", s);
}
```

所权权转移 (move) 是 rust 所有类型的默认语义。

### 11.3 复制语义

rust 中默认是 move 语义，如果需要复制变量，需要显式调用 clone() 方法。但对于一些简单类型，比如整数，bool，让它们在赋值时默认采用复制操作会让语言更简单。(本质是实现了 Copy trait)。

### 11.4 Box 类型

Box 类型永远执行的是 move 语义。

### 11.5 Clone vs Copy

Copy 全称 std::marker::Copy，std::marker 中的 trait 都是特殊的 trait，目前有四个：Copy, Send, Sync, Sized。它们跟编译器密切绑定，这些 trait 内部没有方法，唯一任务是给类型打上标记 (所以 mod 叫 marker)，供编译器使用。

一旦一个类型被标记为 Copy trait，那么它在变量绑定，函数参数传递，函数返回值等场景下，都是 copy 语义，而不是默认的 move 语义。

Copy 的实现条件，并不是所有的类型都可以实现 Copy trait。Rust 规定，对于自定义类型，只有所有成员都实现了 Copy trait，它才有资格实现 Copy trait。

对于数组和元组类型，如果它们内部的每一个元素都是 Copy，那么它们本身则会自动实现 Copy。

对于 struct 和 enum，并不会自动实现 Copy，需要手动实现，但前提是每个元素是 Copy，否则无法实现，比如含有 String, Vec 这样无法 Copy 的类型。

结论：我们可以认为，Rust 中只有 POD (C++ 中 Plain Old Data) 类型才有资格实现 Copy trait。

### 11.6 析构函数

为类型实现析构函数的方法是实现 Drop trait:

```rust
trait Drop {
  fn drop(&mut self);
}
```

drop() 方法由系统自动调用，不能手动调用，如果想提前让变量析构掉，使用标准库的 `std::mem::drop<T>(_x: T) {}` 方法。可以看到这个方法其实没有真正的实现，它只不过是通过转移变量的所有权来提前析构它。

那我这么做是不是也可以啊：

```rust
fn main() {
  let s = String::from("hello");
  ...
  { s; }
  ...
}
```

测试了后是可以的，就是丑了点。

带析构函数的类型是无法满足 Copy 语义的。

## 12. 借用和生命周期

### 12. 借用

借用不拥有所有权，所以才叫 "借"。

&T，&mut T。

注意 mut 和 &mut 的区别。如果 mut 修饰的是变量，那么它代表这个变量可以被重新绑定；如果 mut 修饰的是借用 &，那么它代表的是被引用的对象可以被修改。

示例：

```rust
fn main() {
  let mut var = 0_i32;
  {
    let p1 = &mut var; // p1 指针本身是非 mut 的，不能被重新绑定，但可以通过 p1 改变 var 的值
    *p1 = 1;
    println!("{}", p1);
    println!("{}", var);
  }
  {
    let temp = 2_i32;
    let mut p2 = &var; // 我们不能通过 p2 改变 var 的值，但 p2 指针本身指向的位置可以改变
    p2 = &temp;
    println!("{}", p2);
    println!("{}", var);
  }
  {
    let mut temp = 3_i32;
    let mut p3 = &mut var; // 我们既可以通过 p3 修改 var 的值，也可以改变 p3 自身指向的位置
    *p3 = 5;
    p3 = &mut temp;
    println!("{}", p3);
    println!("{}", var);
  }
}
```

借用指针在编译后就是一个普通的指针。

### 12.3 借用规则

详略。

变量被借用后 (无论是只读借用还是可变借用)，自身会被冻结，无法被修改，除非等借用被取消后。

### 12.4 生命周期标记

(迄今为止还是很迷糊的一个点，可能要到实践时才能彻底理解。)

## 13. 借用检查

...共享不可变，可变不共享。

## 14. NLL (Non-Lexical-Lifetime)

默认变量的生命周期，起始于定义，终止于作用域 (词法作用域) 结束的地方。这样的设定过于严格，会让很多实际正确的代码无法通过编译。而 NLL 则是让借用的生命周期不要过长 (即小于词法作用域)，适可而止，避免不必要的编译错误。

(哦，想起了，之前遇到发现一段代码里对同一个变量有多个可变借用，却通过了编译，原来就是 NLL 起了作用啊，因为这些可变借用的生命周期在最后一次使用时就结束了，而不是到作用域才结束。)

## 15. 内部可变性

Cell/RefCell/UnsafeCell

一般情况下，一个对象要么整体可以被修改 (用 mut 修饰)，要么整体不可修改 (默认情况)，不可能说对象中的一部分属性可修改，另一部分不可修改。mut 用来修饰整个对象，而不能在定义类型时用来修饰部分属性。

而内部可变性就是用来满足使对象在不使用 mut 修饰的情况，内部属性可以改变的需求。

示例：

```rust
use std::cell::Cell;

fn main() {
    let data: Cell<i32> = Cell::new(100);
    let p = &data;
    data.set(10);
    println!("{}",p.get());  // 10
    p.set(20);
    println!("{:?}", data);  // Cell { value: 20 }
}
```

上例中，有多个修改源，且修改源没有用 mut 修饰，这就是所谓的内部可变性。

具体为什么上面的代码可行，是因为 Cell 实际只是一个壳，它把数据严严实实地包裹在里面，所有的指针只能指向 Cell，不能直接指向数据，修改数据只能通过 Cell 来完成，用户无法创造一个直接指向数据的指针。因此，它并不会造成内存安全问题...

RefCell/UnsafeCell 暂不理解，先跳过。

## 16. 解引用

引用的反操作，和 c/c++ 一样，使用 `*` 操作符。

```rust
fn main() {
    let v1 = 1;
    let p1 = &v1;
    let v2 = *p1;
    println!("{} {}", p1, v2); // 1 1
}
```

### 16.1 自定义解引用

实现 std::ops::Deref 或 std::ops::DerefMut 这两个 trait。

### 16.2 自动解引用

当编译器找不到实例的某个方法后，会自动尝试使用 deref 方法后再找该方法...

其余略，暂不理解，先跳过。

## 17. 泄漏

跳过，rust 无法避免，内存泄漏并不是内存不安全。

## 18. Panic

## 19. Unsafe

## 20. Vec 源码分析

---

## 21. 泛型

这里面讲到了关联类型的一些局限性，有空再详细看。

## 22. 闭包

三种捕获方式这里解释得很详细，很好理解。

剩下的慢慢看吧，知识点有点多，一口没法吃成胖子。
