# 《Rust 编程之道》Note - 4

- 第十二章 元编程
- 第十三章 超越安全的边界

## 第十二章 元编程

元编程：用代码生成代码。

元编程技术分类：

1. 简单文本替换：c/c++ 的宏定义
1. 类型模板：c++ 的模板
1. 反射：ruby/java/go/rust 或多或少都支持反射，在运行时或编译时获取程序内部信息
1. 语法扩展：ruby/elixir/rust 等语言可以对抽象语法树进行操作而扩展语言的语法
1. 代码自动生成：go generate 命令将注释自动生成代码

rust 支持反射和 AST 语法扩展两种手段。

### 12.1 反射 (reflect)

反射机制，程序自我访问、检测和修改自身状态或行为的能力。Rust 标准库提供了 std::any::Any 来支持运行时反射。

主要是提供了一个：

- is() 方法判断类型
- `downcast_ref()` 和 `downcast_mut()` 方法用于将泛型 T 向下转换成具体的类型

详略，需要时再细看。

### 12.2 宏系统 (macro)

宏即批处理，根据预定义的规则转换成相应的输出，这个转换过程叫宏展开。

#### 12.2.1 起源

宏操作大致分两种：

- 文本替换：c/c++
- 语法扩展：源于 Lisp 的 S 表达式

示例：

```lisp
(defmacro one! (var)
  (list 'setq var 1)
)

(+ (one! x) 2) // 调用宏
(+ (setq x 1) 2) // 宏展开
```

#### 12.2.2 Rust 中宏的种类

两大类：

- 声明宏 (declarative marcro) - 使用 `macro_rule!` 声明
- 过程宏 (procedural macro) - 实现自定义派生属性，比如 Serde 库实现的 `#[derive(Serialize, Deserialize)]`

宏的使用：

- 调用宏，比如 `println!`，`assert_eq!` 等，通常由声明宏来实现，也可以由过程宏来实现
- 属性宏，形如 `#[derive(Debug)]` 或 `#[cfg]` 这种形式的语法，可以通过过程宏来实现，也可以通过编译器插件实现

宏的来源：

- 内置宏：又有两种，一种是标准库实现，一种是编译器固有行为；
- 自定义宏

一个声明宏的示例：

```rust
macro_rule! unless {
  ($arg: expr, $branch: expr) => (if !$arg { $branch };);
}

fn cmp(a: i32, b: i32) {
  unless!(a>b, {
    println!("{} < {}", a , b);
  })
}
```

#### 12.2.3 编译过程

详略。

#### 12.2.4 声明宏

声明宏的格式：

```rust
macro_rule! $name {
  $rule0;
  $rule1;
  //...
  $ruleN;
}
```

每条 rule 的格式：`( $pattern ) => ( $expansion )`

上例中 `($arg: expr, $branch: expr) => (if !$arg { $branch };);`，`$arg` 表示捕获变量，expr 表示捕获类型，expr 指表达式类型。

其它捕获类型：

- item - 语言项，组成一个 rust 包的基本单元
- block - 代码块，花括号组成的
- stmt - 语句，一般指分号结尾的代码
- expr - 表达式，会生成具体的值
- pat - 模式
- ty - 类型
- ident - 标识符
- path - 路径，比如 std::iter 等
- meta - 元信息，表示包含在 `#[...]` 属性内的信息
- tt - TokenTree 缩写
- vis - 可见性，比如 pub
- lifetime - 生命周期参数

声明宏的实现技巧，以 `hashmap!()` 的实现为例。

使用：

```rust
fn main() {
  let map = hashmap!{
    "a" => 1,
    "b" => 2
  }
  assert_eq!(map["a"], 1);
}
```

实现：首先区配 `key => value` 的定义格式，可以用 `$key: expr => $value: expr`，另外要求可以重复匹配，Rust 的宏支持重复匹配，格式是 `$(...) sep rep`。seq 表示分隔符，rep 表示重复次数，可以用数据或 `*`, `+`。

初版实现：

```rust
macro_rule! hashmap {
  ($($key:expr => $value:expr),*) => {
    let mut _map = ::std::collections::HashMap::new();
    $(
      _map.insert($key, $value);
    )*
    _map
  }
}
```

初版有个问题，hashmap 最后一项不能用逗号结尾，优化，详略，需要时再看吧。

调试宏：详略。

声明宏的卫生性：即不会污染原来的词法作用域，详略。

宏的导入导出：详略。

#### 12.2.5 过程宏

过程宏可以实现三种类型的宏：

1. 自定义派生属性，类似 `#[derive(Debug)]` 这种
1. 自定义属性，类似 `#[cfg()]` 这种
1. Bang 宏，和 `macro_rule!` 定义的宏类似

示例，详略。

### 12.3 编译器插件

暂时先跳过。

web 框架 rocket 实现了很多自定义属性。

## 第十三章 超越安全的边界

### 13.1 Unsafe Rust

Unsafe Rust 是 Safe Rust 的一个超集，也就是说，在 Unsafe Rust，并不会禁用 Safe Rust 中的任何安全检查。

Unsafe Rust 是指进行下面五种操作时，不会提供任何安全检查：

1. 解引用裸指针
1. 调用 unsafe 的函数或方法
1. 访问或修改可变静态变量
1. 实现 unsafe trait
1. 读写 Union 联合体中的字段

#### 13.1.1 Unsafe 语法

- unsafe 关键字，用于标记函数、方法和 trait
- unsafe 块，用于执行 Unsafe Rust 允许的五种操作

#### 13.1.2 访问和修改可变静态变量

略。

#### 13.1.3 Union 联合体

略。

#### 13.1.4 解引用原生指针

原生指针 (裸指针)：`*const T`, `*mut T`。

原生指针特点：

- 并不保证指向合法的内存
- 不能像智能指针那样自动清理内存
- 没有生命周期概念
- 不能保证线程安全

详略。

### 13.2 基于 Unsafe 进行安全抽象

通过 unsafe 关键字和 unsafe 块可以执行一些跳过安全检查的特定操作，但并不代表使用了 unsafe 就不安全。

#### 13.2.1 原生指针

- 在需要时跳过 Rust 安全检查
- 与 C 语言打交道

标准库为原生指针内建了很多方法和函数，比如：

- std::ptr::null() / is_null()
- offset()
- read() / write()
- replace() / swap()

详略。

使用原生指针进行安全抽象：在标准库中有很多方法是基于 Unsafe Rust 实现的安全抽象，比如 `Vec<T>` 动态数组的 insert() 方法，用 Safe Rust 无法实现 (啊? soga)。详略。

#### 13.2.2 子类型与型变

子类型，简单理解成 subclass 就行。一般说来，可以用父类型的地方，就可以用子类型替代。(里氏替换原则)

型变，有三种：

- 协变。可以继续保持子类型关系，比如 Cat 是 Animal 的子类型，那么 `List<Cat>` 也是 `List<Animal>` 的子类型。
- 逆变。和协变正好相反。
- 不变。既不保持，也是逆转，即 `List<Cat>` 和 `List<Animal>` 没有关系。

Rust 中只有生命周期具有子类型关系。(?? trait 不是可以继承吗??) 如果有生命周期满足 `'long:'short` 这样的关系，那么可以说 'long 是 'short 的子类型，比如 &'static 是 &'a str 的子类型。

了解由生命周期组成的复合类型，具体什么样的型变规则很重要。

举例，略。Rust 中大部分结构都是协变的。如果没有合理利用协变，将会产生未定义行为的风险，解决办法，将协变改成逆变或不变。

使用 `PhantomData<T>`

`PhantomData<T>` 是一个零大小类型的标记结构体，也叫作 "幻影类型"，在需要指定一个并不使用的类型时就可以使用它。除此之外，它还可以扮演以下三种角色：

- 型变：可以产生协变、逆变和不变三种情况
- 标记拥有关系。和 drop 检查有关
- 自动 trait 实现。比如 Send 和 Sync

具体使用，详略，需要时再看。

协变、逆变与不变类型列表，有点小复杂，暂时先跳过吧，需要时再看。

#### 13.2.3 未绑定生命周期

Unsafe 代码很容易产生未绑定生命周期，导致垂悬指针，要避免。

#### 13.2.4 Drop 检查

先跳过吧，有点不明白。

解决办法：

- `#[may_dangle]` 属性
- `PhantomData<T>`

使用 std::mem::forget 阻止析构函数被调用。比如用在和 c 语言交互时需要这么做。详略。

#### 13.2.5 `NonNull<T>` 指针

实际上是一种特殊的 `*mut T` 原生指针，它的特殊之处有两点：协变和非零，旨在成为 Unsafe Rust 默认的原生指针。

详略，需要时再细看。

#### 13.2.6 Unsafe 和恐慌安全

略。

#### 13.2.7 堆内存分配

在 Unsafe Rust 中需要手动进行堆内存分配，所以 Rust 标准库 std::alloc 模块提供了堆内存分配相关的 API。

最重要的是 GlobalAlloc trait，两个重要方法 alloc 和 dealloc。

详略。

#### 13.2.8 混合代码内存安全架构三大原则

略。

### 13.3 和其它语言交互

#### 13.3.1 外部函数接口

FFI (Foreign Function Interface)。

ABI (Application Binary Interface)

- 调用约定
- 内存布局
- 处理器指令集
- 目标文件和库的二进制格式

C 语言 ABI 是唯一通用的稳定的标准 ABI。

在 Rust 中使用 FFI，通过 extern 关键字和 extern 块。

静态库，动态库。

交叉编译。

extern 语法。

#### 13.3.2 与 c/c++ 语言交互

Rust 可以无缝地调用 C 函数。通过 C-ABI，Rust 也可以被其它语言调用。

详细内容先跳过，需要时再回头细看。

#### 13.3.3 使用 Rust 提升动态语言性能

使用 Rust 为 Ruby/Python/Node.js 等动态语言编写本地扩展。

详略。需要时再细看。

### 13.4 Rust 与 WebAssembly

简单了解。

WebAssembly 是近两年兴起的一种**新的字节码格式**，缩写是 WASM。

#### 13.4.1 WebAssembly 要点介绍

- 模块，基本编译单元，一个 .wasm 文件就是一个模块，其中定义了各种函数，可以被 JavaScript 加载调用
- 线性内存，ArrayBuffer，用于和 JavaScript 通信
- 表格，存放函数引用，支持动态调用函数
- 实例
- 栈式机器

文本格式 wast。WebAssembly 提供两种格式：二进制和文本格式。文本格式基于 s 表达式，供人类读写，称为 wast。

手写 wast。详略。

#### 13.4.2 使用 Rust 开发 WebAssmebly

详略。

#### 13.4.3 打造 WebAssembly 开发生态

详略。yew 框架。

## 附录 A - Rust 开发环境指南

略。

## 附录 B - Rust 如何调试代码

LLDB + VS Code，详略，需要时再细看。
