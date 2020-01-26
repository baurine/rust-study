# Rusy by Example notes - 1

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [通过例子学 Rust](https://www.shiyanlou.com/courses/1499)
- [shiyanlou/rust-by-example-cn](https://github.com/shiyanlou/rust-by-example-cn)

Tables:

1. Hello World
2. Primitives
3. Custom Types
4. Variable Bindings
5. Types
6. Conversion
7. Expressions
8. Flow of Control
9. Functions
10. Modules
11. Crates
12. Cargo
13. Attributes

## 1. Hello World

注释的几种方式：

1. `//`
1. `/* ... */`
1. `///`
1. `//!`

后面两种可以配合 `rustdoc` 命令生成文档。

格式化输出，详见文档：https://doc.rust-lang.org/std/fmt/

常见宏：

1. `format!`
1. `print!`
1. `println!`
1. `eprint!`
1. `eprintln!`

示例：

```rust
fn main() {
  println!("{} days", 31);  // 31 days
  println!("My name is {0}, {1} {0}", "Bond", "James");  // My name is Bond, James Bond
  println!("{a} {b}", a=1, b=2); // 1 2
  println!("{number:>0width$}", number=1, width=6); // 000001
  println!("{:.*}", 3, 3.1415926); // 3.142
}
```

只能 std 库里类型默认实现了打印功能，其余类型必须手动实现。声明 `#[derive(Debug)]` 的类型允许使用 `{:?}` 或 `{:#?}` 输出打印，类似浏览器端的 js 的 `console.log(obj)`，把一个对象的所有属性及其值打印出来。`{:?}` 显示成单行，`{:#?}` 则美化输出。

```rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // common print
    // Person { name: "Peter", age: 27 }
    println!("{:?}", peter);
    // Pretty print
    // Person {
    //     name: "Peter",
    //     age: 27,
    // }
    println!("{:#?}", peter);
    // error
    println!("{}, peter);
}
```

如果想用 `{}` 进行输出，则需要显式的实现 `fmt::Display` trait。

对上面的 Person 实现 `fmt::Display` trait 并用 `{}` 打印。

```rust
use std::fmt

impl fmt::Display for Person<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "name is {} and age is {}", self.name, self.age)
  }
}

fn main() {
  ...
  // name is peter and age is 27
  println!("{}", peter)
}
```

如果想要用 `{:b}` 的格式输出，则需要实现 fmt::Binary trait。

## 2. Primitives

基本类型，tuple，数组与 slices。

主要来看数组与 slice。数组的长度固定，编译期已知，类型签名 `[T; size]`，slices 和数组类似，但长度在编译期未知，它的大小由两个字组成 (胖指针?)，第一个字是指针，指向真正的数据所在内存地址，第二个字是数据长度，类型签名 `&[T]`。slices 可以用来借用数组的一部分。

```rust
fn analyse_slice(slice: &[i32]){
    println!("first el: {}", slice[0]);
    println!("len: {}", slice.len());
}

fn main() {
    let xs = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    analyse_slice(&xs); // &xs == &xs[..]
    analyse_slice(&ys[1..5]);
}
```

## 3. Custom Types

struct 和 enum。

没有新知识点，略过。

示例：用 enum 实例了 linked-list，使用了递归...

## 4. Variable Bindings

使用 let 进行变量绑定，略。

## 5. Types

类型转换，rust 没有隐式转换，所有转换必须显式声明，使用 as 关键字。

```rust
fn main() {
    let decimal = 65.4321_f32;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    // Casting: 65.4321 -> 65 -> A
}
```

别名 (alias)，使用 type 关键字，主要用来简化太长的类型书写，比如 `type IoResult<T> = Result<T, IoError>`。

## 6. Conversion

(又是类型变换?? 不是，是说由 type A 生成 type B，或者从 type B 变回 type A)

最典型的，str 和 String。

```rust
let my_str = "hello";  // my_str 是 &str 类型
let my_string = String::from(my_str);
```

From trait 声明了 from() 方法。当你为某个类型实现了 From trait 后，编译器会自动为目标类型实现 Into trait，它包含 into() 方法。

示例：

```rust
fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{}", my_string); // hello

    // 必须显式声明 my_string_2 为 String 类型，否则编译出错
    let my_string_2: String = my_str.into();
    println!("{}", my_string_2); // hello
}
```

TryFrom 和 TryInto trait，用于可能转换失败的情况，返回值是 `Resut<T, Error>`。

实现了 fmt::Display trait 的类型会自动实现 ToString trait，并能使用 `to_string()` 方法。

解析字符串，实现 FromStr trait 的 parse() 方法。

## 7. Expressions

语句和表达式，略。粗略区分就是 ';' 结尾的是语句 (除了 return 语句)，返回 () 类型，没有 ';' 结尾的是表达式，返回的是其求的值。

## 8. Flow of Control

if/else, loop, while, for...in, match, if let, while let。

在循环中可能通过 `break return_value;` 来直接返回值。

for and range。range: `1..101`，`1..=100`

for and iterators。类似三种闭包 (Fn, FnMut, FnOnce)，迭代器也有三种：

- `iter()`: 引用
- `iter_mut()`: 可变引用
- `into_iter()`: 值，可能会 move 所有权

示例暂略。

## 9. Functions

闭包，也称 lambda 表达式。`|val| val+x`，相比函数具有更强大的类型自动推导。

### 闭包 - 捕获

闭包可以捕获变量：

- 通过引用：`&T`
- 通过可变引用：`&mut T`
- 通过值：`T` (可能会 move 所有权)

它们更倾向于通过引用 `&T` 来捕获变量并且只在需要时才用后面用法。

我的理解：

- 如果在闭包中只访问了外部变量，那么 `&T`, `&mut T`, `T` 三种捕获方式其实都是可能的，但优先考虑 `&T` 的捕获方式
- 如果在闭包中修改了外部变量，那么 `&mut T`, `T` 两种捕获方式都是可能的，但优先考虑 `&mut T` 的捕获方式
- 如果在闭包中释放了外部变量，那么只有 `T` 一种捕获方式

如果对闭包使用 move 声明，则确定使用 `T` 捕获方式，即使在闭包内形式上只访问了外部变量。捕获的外部变量的所有权将转移到闭包内部。

示例：

```rust
fn main() {
    use std::mem;

    let color = "green";
    // 闭包捕获了外部的 color 变量，且闭包内部对它只读，使用借用 `&T` 捕获方式
    let print = || println!("`color`: {}", color);
    // Call the closure using the borrow.
    print();
    print();

    let mut count = 0;
    // 闭包捕获了外部的 count 变量，且对它进行了修改，使用 `&mut T` 和 `T` 的捕获方式皆可，
    // 但 `&mut T` 的方式限制小一些，优先考虑
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    // Call the closure.
    inc();
    inc();
    // 还可以继续可变借用
    let _reborrow = &mut count;
    // 但是后面不能再调用 inc() 了，因为如果再次调用 inc()
    // 相当于有多个 `&mut T`，这是不允许的
    // 同一时刻只能有一个 `&mut T`
    // inc();

    // A non-copy type.
    let movable = Box::new(3);
    // `mem::drop` 需要 `T` 方式，所以这个闭包使用 `T` 捕获方式
    // movable 的所有权会转移到闭包中
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    // `consume` 只能被调用一次
    consume();
    // 再次调用时会导致编译不通过
    // consume();

    ////////////////////
    ///
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];
    // 闭包捕获了 haystack 并对它只读，但由于使用了 move 声明，则使用 `T` 方式捕捉
    // haystack 的所有权将转移到 contains 闭包中
    let contains = move |needle| haystack.contains(needle);
    // 如果继续访问 haystack 将会报错
    // println!("There're {} elements in vec", haystack.len());
}
```

### 闭包 - 作为参数

三种类型：

- Fn: 通过引用
- FnMut: 通过可变引用
- FnOnce: 通过值 (可能会 move 所有权)

我的理解：

- 如果参数声明成了 FnOnce，但实际使用时，闭包可以按 `&T`, `&mut T`, `T` 三种方式捕获外部变量，按最小限制的原则选择，比如如果实际实现只读了外部变量，那么会用 `&T` 捕获，如果修改了外部变量，则按 `&mut T`，否则按 `T`
- 如果参数声明成了 FnMut，则闭包可以按 `&T`, `&mut T` 两种方式捕获外部变量
- 如果参数声明成了 Fn，则闭包只能按 `&T` 捕获外部变量

示例：

```rust
// 这个函数接收 FnOnce 类型的闭包作为参数
fn apply<F>(f: F) where
    F: FnOnce() {
    // 如果把 FnOnce 改成 Fn/FnMut，编译出错
    f();
}

// 这个函数接收 Fn 类型的闭包作用参数
fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    // 如果把 Fn 改成 FnMut/FnOnce，编译正常
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // diary 闭包按引用方式捕获了 greeting 外部变量
    // 按值方式捕获了 farewell 外部变量
    let diary = || {
        // 引用方式 - Fn
        println!("I said {}.", greeting);

        // 可变引用 - FnMut
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 按值捕获 - FnOnce
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
```

### 闭包 - 类型匿名

使用闭包作为参数时，为什么需要使用范型。因为 Fn/FnMut/FnOnce 是 trait 而不是 strcut。

当定义一个闭包时，编译器将隐式地创建一个新的匿名结构体来存储内部的捕获变量， 同时针对此未知类型通过其中的一种 trait：Fn，FnMut，或 FnOnce 来实现功能。

### 闭包 - 形参为闭包，实参为函数

当形参为闭包时，实参传递一个函数也是可以的 (内部会做一层转换吗? 将这个函数用一个闭包封装一下?)

### 闭包 - 作为函数的返回类型

闭包作用返回值也是可以的，但稍为特殊一点的是，必须使用 move 获得外部变量的所有权。

```rust
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
```

### Diverging functions

never type，使用 `!` 表示。示例：

```rust
fn foo() -> ! {
    panic!("This call never returns.");
}
```

## 10. Modules

模块，mod，内部成员默认可访问性为 private，用 pub 声明覆盖默认的 private。

use / as, self / super 用法，详略。

## 11. Crates

在当一个 crate 库中要链接另一个 crate 时，使用 `extern crate xxx;`...

## 12. Cargo

如果 cargo build 不能完全满足需求，可以自己实现一个编译脚本，并在 Cargo.toml 中声明：

```toml
[package]
...
build = "build.rs"
```

其余略。

## 13. Attributes

类似 java 的注解和 c/c++ 的预编译指令。

在 crate 中使用 `#![crate_attribute]` 的语法，在其它地方使用 `#[attribute]` 的语法。

attribute 声明可以接受参数，几种语法形式：

- `#[attribute = "value"]`
- `#[attribute(key = "value")]`
- `#[attribute(value)]`

一些 attributes:

- 允许死代码：`#[allow(dead_code)]`
- cfg: `#[cfg(xxx)]`
- `#[derive(Debug, PartialEq)]`
