fn main() {
    // 变量和可变性
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 数据类型
    let _integer: i32 = -10;
    let _unsigned_integer: u32 = 10;
    let _float: f64 = std::f64::consts::PI;
    let _boolean: bool = true;
    let _character: char = 'a';

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    // 数组
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The first element of the array is: {}", first);
}