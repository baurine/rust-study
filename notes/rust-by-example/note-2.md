# Rusy by Example notes - 2

Tables:

14. Generics
15. Scoping rules
16. Traits
17. macro_rules!

## 14. Generics

```rust
struct SGen<T>(T);

fn generic<T>(_s: SGen<T>) {
  ...
}

fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

rust 中的范型 T，表示的是 struct 或基本类型，并不表示 trait，因此，如果要限制某个类型必须实现了某种 trait，使用 `T: Display` 这样的语法。

疑惑：`fn printer<T: Display>(t: T) {...}` 为什么不直接用 `fn printer(d: Display) {...}`。

解答：涉及静态分发与动态分发的问题，前者为静态分发，后者为动态分发 (多态)。

### 关联项 - 关联类型

理解它解决的问题，才能理解它存在的意义。

简化泛型重复声明的问题。

看代码理解：

```rust
struct Container(i32, i32);

// 这个 trait 检查 2 个项是否存到 Container（容器）中。
// 还会获得第一个值或最后一个值。
trait Contains {
    // 在这里定义可以被方法利用的泛型类型。
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型
    // 为 `Container(i32, i32)`，那么 `output`（输出）类型
    // 会被确定为 `i32` 和 `i32`。
    type A = i32;
    type B = i32;

    // `&Self::A` 和 `&Self::B` 在这里也是有效的。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 得到第一个数字。
    fn first(&self) -> i32 { self.0 }

    // 得到最后一个数字。
    fn last(&self) -> i32 { self.1 }
}
```

### Phantom type parameters

暂时不理解它存在的意义，跳过。

## 15. Scoping rules

- 所有权
- 借用
- 生命周期

没有完全消化。

## 16. Traits

> A trait is a collection of methods defined for an unknown type: Self.

Self 表示目标 struct。trait 中的方法可以有默认实现。

```rust
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}
```

### Returing trait with dyn

和其它语言不一样，你不能在 rust 中直接返回一个 trait 类型，比如 `fn random_animal() -> Animal {...}`，因为 rust 要求返回值的大小在编译期是已知的。

一个 workaround 是返回一个 Box，在 Box 中存放 Animal 的引用。当返回值是指向分配在堆区的内存时，rust 要求加上 dyn 关键字显示声明，因此，完整的定义是这样的：`fn random_animal() -> Box<dyn Animal> {...}`。

### impl Trait

如果返回值实现了某个 trait，比如 MyTrait，则返回值可以声明为 `impl MyTrait`... (那怎么在编译期确认它的大小?)

一个例子就是前面讲过的，闭包作为返回值。

```rust
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}
```

## 17. macro_rules!

rust 中可以借助宏实现元编程，宏在编译期展开变成内联代码，且它是展开成 AST，而不是像 c 那样简单的替换展开，因此不会像 c 那样出错很多意料之外的错误。

一个示例，用 `macro_rules!` 定义一个宏，用宏名加上 `!` 调用它。

```rust
// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}
```

### 宏语法 - Designators

- Patterns and Designators
- Overloading
- Repetition

宏定义中，用 `$` 作为前缀声明参数，参数后用 `:` 加指示符 (designator) 声明类型。这个类型并不是 rust 像 u32, String 这种类型，而是 ident, expr 之类的，具体含义暂不理解。

- ident: 表明参数是函数或变量名字，表示这个宏将生成可调用的函数或变量
- expr: 表明参数是表达式
- block: (暂不知道)

### 宏语法 - Overload

宏可以重载，接受参数的不同组合 (类似 Elixir 中的语法)

### 宏语法 - Repeat

宏在参数列表中可以使用 + 来表示一个参数可能出现一次或多次，使用 \* 来表示该参数可能出现零次或多次。

先只作了解，需要时再详细学习。

宏可以用来实现 DSL。

18 章后面的先跳过了，知识点在 《the rust programming book》都学习过了。需要时再仔细学习。

Rust 的关键在于对所有权的熟练掌握。`&T`, `&mut T`, `T`，其余的和其它语言都是相通的。
