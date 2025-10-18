# 枚举

枚举（enum）是一种允许我们定义一个包含一组变体的类型的类型。每个变体都可以有不同的类型和数据量。

## 定义枚举

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

我们可以使用更简洁的方式来表达相同的概念，即将数据直接附加到枚举的每个变体中。

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

## `Option` 枚举

`Option` 是标准库定义的另一个枚举。它在 Rust 代码中非常普遍，因为它编码了一个非常常见的场景，即一个值可以是某个值，也可能什么都不是。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>` 枚举非常有用，以至于它甚至被包含在 prelude 中；你不需要显式地导入它。此外，它的变体也是如此：你可以直接使用 `Some` 和 `None`，而无需 `Option::` 前缀。