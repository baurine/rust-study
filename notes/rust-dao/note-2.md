# 《Rust 编程之道》Note - 2

- 第六章 函数、闭包和迭代器
- 第七章 结构化编程
- 第八章 字符串与集合类型

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
