fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    takes_ownership(s1);

    // println!("{}", s1); // This would cause a compile error because s1's ownership has been moved.

    let x = 5;
    makes_copy(x);
    println!("x = {}", x);
}

fn takes_ownership(some_string: String) {
    println!("Inside takes_ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Inside makes_copy: {}", some_integer);
}
