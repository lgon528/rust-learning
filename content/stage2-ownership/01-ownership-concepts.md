# 所有权概念

Rust 的核心特性是所有权（ownership）。所有权是一组规则，用于管理内存。它使 Rust 能够在没有垃圾回收器的情况下保证内存安全，从而在编译时消除与内存相关的错误。

## 所有权规则

1.  **每个值都有一个所有者。**
2.  **一次只能有一个所有者。**
3.  **当所有者超出作用域时，该值将被丢弃。**

## 变量作用域

作用域是程序中一个项（item）有效的范围。例如：

```rust
{
    let s = "hello"; // s 从这里开始有效

    // 对 s 执行操作
} // s 的作用域到此结束，s 不再有效
```

## `String` 类型

为了说明所有权规则，我们需要一个比我们在基本语法部分看到的更复杂的数据类型。`String` 类型是在堆上分配的，因此它能够存储在编译时未知的文本量。

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() 在字符串后附加字面值

println!("{}", s); // 这将打印 `hello, world!`
```

## 移动

当我们将一个 `String` 赋给另一个变量时，所有权会发生转移，这称为“移动”（move）。

```rust
let s1 = String::from("hello");
let s2 = s1;

// println!("{}", s1); // 这将导致编译错误，因为 s1 的所有权已移动到 s2
```

## 克隆

如果我们确实需要深度复制 `String` 的堆数据，而不仅仅是栈数据，我们可以使用 `clone` 方法。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

## 函数与所有权

将值传递给函数与将值赋给变量类似。将变量传递给函数将移动或复制，就像赋值一样。

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该会移动到函数里，
                                    // 但 i32 是 Copy 的，所以没关系，
                                    // 在这里仍然可以使用 x

} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop`。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```