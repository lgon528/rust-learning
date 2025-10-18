# 结构体

结构体（struct）是一种自定义数据类型，允许我们将多个相关的值组合成一个有意义的组。与元组（tuple）类似，结构体的各个部分可以是不同的类型。但与元组不同，结构体需要为每个部分命名，以便清楚地表明值的含义。

## 定义和实例化结构体

使用 `struct` 关键字定义结构体，并为整个结构体提供一个名称。然后，在大括号内，为所有字段定义名称和类型。

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

## 结构体更新语法

当我们需要从现有实例创建新实例时，可以使用结构体更新语法（struct update syntax）。

```rust
// let user2 = User {
//     email: String::from("another@example.com"),
//     username: String::from("anotherusername567"),
//     ..user1
// };
```

## 元组结构体

元组结构体（tuple structs）是一种定义结构体的方式，它看起来像元组。元组结构体有结构体名称，但其字段没有名称。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## 类单元结构体

我们还可以定义没有任何字段的结构体！这些被称为类单元结构体（unit-like structs），因为它们类似于 `()`，即单元类型。类单元结构体在我们需要在某个类型上实现 trait 但没有任何数据要存储的情况下很有用。