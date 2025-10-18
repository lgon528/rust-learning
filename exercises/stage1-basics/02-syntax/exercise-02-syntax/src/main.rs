fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let a = [1, 2, 3, 4, 5];

    println!("x: {}, tup.1: {}, a[0]: {}", x, tup.1, a[0]);
}
