# 《深入浅出 Rust》Note

第一部分 - 基础知识 - 1 ~ 9 章

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

## 3. 语句和表达式

- 语句 (Statement)，不产生值，返回值是 ()，带分号
- 表达式 (Expression)，产生值，有类型，不带分号

if-else, loop, while, for ...

## 4. 函数

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

### 4.3 main() 函数

Rust 中的 main() 函数是无参的，运行时传进来的参数通过 std::env::args() 获取，而不从 main() 函数的参数中获取，获取环境变量通过 std::env::var() 和 std::env::vars() 方法。

### 4.4 const fn

编译期执行的函数，返回值为编译期常量。

### 4.5 函数递归调用

```rust
fn fib(n: u32) -> u32 {
  if n == 1 || n == 2 {
    n
  } else {
    fib(n-1) + fib(n-2)
  }
}

fn main() {
  println!("{}", fib(8));
}
```

Rust 还没有实现尾递归优化。

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

可以 `impl traitA for traitB`，为 trait 实现另一个 trait，这个在后面还会详述。

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

### 5.4 完整函数调用语法

如果一个 struct 实现了两种 trait，但这两种 trait 声明了相同的函数，在调用这个函数时，需要这样使用：

```rust
trait Cook {
  fn start(&self);
}
trait Wash {
  fn start(&self);
}
struct Chef;
impl Cook for Chef {...}
impl Wash for Chef {...}

fn main() {
  let me = Chef;
  me.start() // 有歧义
  // 正确使用
  <Cook>::start(&me);
  <Chef as Wash>::start(&me);
}
```

### 5.5 trait 约束和继承

一般情况下 trait 不能直接用于声明变量或参数的类型，只能用来在泛型用来约束泛型的类型。

```rust
fn my_print<T : Debug>(x: T) {
  println!("The value is {:?}.", x);
}
// or
fn my_print<T>(x: T) where T: Debug {
  println!("The value is {:?}.", x);
}
```

trait 之间可以继承，struct 实现子 trait 时，也要同时实现父 trait。

```rust
trait Base {}
trait Derived : Base {}
struct T;
impl Derived for T {...}
// 同时需要下面这句编译才能通过
impl Base for T {...}
```

### 5.6 Derive

Rust 里面为类型 impl 某些 trait 的时候，逻辑是非常机械化的。为许多类型重复而单调地 impl 某些 trait，是非常枯燥的事情。为此，Rust 提供了一个特殊的 attribute，它可以帮我们自动 impl 某些 trait。

```rust
#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Foo {
  data : i32
}
```

### 5.7 trait 别名

```rust
trait HttpService = Service<Request = http::Request,
        Response = http::Response,
        Error = http::Error>;
```

### 5.8 标准库中常见的 trait 简介

- Dispaly / Debug: 用于 println!
- PartialOrd / Ord / PartialEq / Eq: 用于浮点数/整数间的比较
- Sized: 用来给编译器进行标记的，官方教程解释得比较清楚
- Default

## 6. 数组和字符串

### 6.1 数组

数组的定义：`[T; n]`，长度固定。相对的 `[T]` 是动态大小类型 (Dynamic Size Type - DST)，`&[T]` 是切片，是胖指针，大小固定 (2 usize)。

对数组进行遍历，数组本身没有实现迭代器 trait，但数组切片实现了，因为可以对数组切片使用 for...in

```rust
fn main() {
  let v = [0_i32; 10];
  for i in &v {
    println!("{}", i);
  }
}
```

(`&v` 在这里是数组切片，不是借用，怎么区分？ -> 两者等价)

数组切片：对数组进行借用操作，可以生成一个数组切片，类型变为 `&[T]`。

切片是胖指针。胖指针点两个 uszie，一个 usize 指向源数组地址，一个 usize 为数组长度。

Range: `1..10` 是 `Range { start: 1, end: 10 }` 的语法糖。Range 实现了迭代器 trait，所以可以直接在它上面使用 for...in

### 6.2 字符串

#### 6.2.1 &str

主要两种类型：&str 和 String。

str 是 Rust 内置类型，&str 是对 str 的借用，也是它的切片。Rust 的字符串内部使用 utf-8 编码 (应该是指 unicode 吧，utf-8 是存储和传输用的，内存中应该是 unicode)，一个 char 占 4 个字节 (4 u8)，所以 Rust 中字符串不能视为 char 类型的数组，而更接近是 u8 类型的数组。

这样设计的一个缺陷是，不能支持 O(1) 时间复杂度的索引操作，即要找第 n 个字符，不能用 `s[n]` 获得。因为 utf-8 是变长编码，所以必须从头遍历，时间复杂度是 O(n)。相应的代码：`s.chars().nth(n)`。

#### 6.2.2 String

&str 无法对指向的字符串进行扩容 (即使是 `&mut str` 也不行)，但 String 可以。String 是在堆上申请的空间，&String 会被编译器自动转换成 &str 类型。

```rust
fn main() {
  let mut s = String::from("hello");
  s.push(' ');
  s.push("world");
  println!("{}", s);
}
```

## 7. 模式解构

解构，在函数参数中也能直接用解构。

匹配，match, `_`, if-let, while-let

匹配中使用 ref 和 mut。(后面还会讲到)

## 8. 深入类型系统

Rust 的类型系统实际是代数类型系统。

一个类型所有取值的可能性叫作这个类型的 “基数” (cardinality)。

Never Type (!):

- 运行时不存在
- 占用空间为 0
- 不能返回
- 可能被转换成任意类型

Never Type，单元类型 `()` 用来统一类型。

借助 `()` 和 HashMap 实现 HashSet，设置其 value 为 `()`。

```rust
pub struct HashSet<T, S = RandomState> {
  map: HashMap<T, (), S>,
}
```

Option 类型，解决 null 问题。

## 9. 宏

先暂时了解，举了一个实现类似 `vec!` 宏的 `hashmap!` 宏的例子。

宏的作用：

- 实现编译期检查
- 实现编译期计算
- 实现自动代码生成
- 实现语法扩展

一个示例：

```rust
let counts = hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];

macro_rule! hashmap {
  ($($key: expr => $val: expr), *) => {{
    let mut map = ::std::collections::HashMap::new();
    $( map.insert($key, $val); )*
    map
  }}
}
```

---

第二部分 - 内存安全 - 2 ~ 10 章

## 10. 内存管理基础

内存安全的概念介绍。

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

为什么需要借用，因为如果一个变量只能有唯一一个入口可以访问的话，程序就太难写了 (恐怕是几乎不可能)。

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

...共享不可变，可变不共享。可以同时有多个不可变借用，但只能有一个可变借用。

## 14. NLL (Non-Lexical-Lifetime)

默认变量的生命周期，起始于定义，终止于作用域 (词法作用域) 结束的地方。这样的设定过于严格，会让很多实际正确的代码无法通过编译。而 NLL 则是让借用的生命周期不要过长 (即小于词法作用域)，适可而止，避免不必要的编译错误。

(哦，想起了，之前遇到发现一段代码里对同一个变量有多个可变借用，却通过了编译，原来就是 NLL 起了作用啊，因为这些可变借用的生命周期在最后一次使用时就结束了，而不是到作用域才结束。)

## 15. 内部可变性

(15，16 章两章有点晦涩)

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

并不是直接修改内部数据，只能通过调用方法修改。

RefCell/UnsafeCell 暂不理解，先跳过。

> 如果你只需要整体性地存入、取出 T，那 么就选 Cell。如果你需要有个可读写指针指向这个 T 修改它，那么就选 RefCell。

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

### 16.5 智能指针

`Rc<T>`, `Arc<T>`，引用计数。T 必须符合上一章所说的内部可变性，即 Cell/RefCell 等类型，比如 `Rc<RefCell<Vec<isize>>>`。

Cow: Copy on Write。

详略，还没太明白，用到时再来仔细消化。

## 17. 泄漏

跳过，rust 无法避免，内存泄漏并不是内存不安全。

## 18. Panic

panic，提前终止程序。

## 19. Unsafe

暂时先跳过，有点复杂。

## 20. Vec 源码分析

暂时先跳过。

---

第三部分 - 高级抽象 - 21 ~ 26 章

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

> 对于一个闭包，编译器是如何选择 impl 哪个 trait 呢?答案是，编译器会都尝试一遍，实现能让程序编译通过的那几个。闭包调用 的时候，会尽可能先选择调用 `fn call(&self，args:Args)` 函数，其次尝试选择 `fn call_mut(&self，args:Args)` 函数，最后尝试使用 `fn call_once(self，args:Args)` 函数。这些都是编译器自动分析出来的。

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

```rust
let v = vec![1,2,3,4,5,6,7,8,9];
for i in v {
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

---

第四部分 - 线程安全 - 27 ~ 31 章

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
