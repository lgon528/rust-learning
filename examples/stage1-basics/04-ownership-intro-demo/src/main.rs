fn main() {
    // 所有权
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // 这行代码会报错，因为 s1 的所有权已经转移给了 s2
    println!("{}", s2);

    // 克隆
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // 函数与所有权
    let s4 = String::from("world");
    takes_ownership(s4);
    // println!("{}", s4); // 这行代码会报错，因为 s4 的所有权已经转移给了 takes_ownership 函数

    let x = 5;
    makes_copy(x);
    println!("{}", x); // i32 是 Copy 类型的，所以 x 仍然有效
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}