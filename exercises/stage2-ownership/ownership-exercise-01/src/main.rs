fn main() {
    let mut s = String::from("hello");

    inspect(&s);
    change(&mut s);
    inspect(&s);

    if eat(s) {
        println!("Ate the string!");
    }

    // inspect(&s); // This would cause a compile error because s has been moved.
}

fn inspect(s: &String) {
    println!("Inspecting string: \"{}\", length: {}", s, s.len());
}

fn change(s: &mut String) {
    s.push_str(" world");
}

fn eat(s: String) -> bool {
    println!("Eating string: \"{}\"", s);
    true
}
