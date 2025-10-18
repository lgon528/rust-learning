# 借用和引用

在 Rust 中，借用（borrowing）允许我们创建对值的引用（references），而无需获取其所有权。这使我们能够在不转移所有权的情况下访问数据。

## 引用

引用允许我们引用某个值，而无需获取其所有权。我们使用 `&` 符号创建引用。

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## 可变引用

我们可以使用 `&mut` 创建可变引用，以允许我们修改借用的值。

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## 借用规则

1.  **在任何给定时间，要么只能有一个可变引用，要么只能有任意数量的不可变引用。**
2.  **引用必须始终有效。**

这些规则在编译时强制执行，以防止数据竞争。

## 悬垂引用

在 Rust 中，编译器会确保引用永远不会成为悬垂引用（dangling reference）——一个指向无效内存的指针。

```rust
// fn dangle() -> &String { // dangle 返回一个对 String 的引用
//     let s = String::from("hello"); // s 是一个新字符串

//     &s // 我们返回对 s 的引用
// } // 这里 s 离开作用域，并被丢弃。它的内存被释放。
  // 危险！
```

此代码将无法编译，因为 `s` 在函数结束时被释放，引用将指向无效的 `String`。