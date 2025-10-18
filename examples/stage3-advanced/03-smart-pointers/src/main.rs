//! 智能指针演示
//!
//! 本示例代码旨在演示 Rust 的智能指针，包括：
//! 1.  **`Box<T>`**：用于在堆上分配值。
//! 2.  **`Rc<T>`**：允许多个所有者的引用计数智能指针。
//! 3.  **`RefCell<T>`**：在运行时强制执行借用规则，允许内部可变性。

use std::rc::Rc;
use std::cell::RefCell;

// 1. `Box<T>` - 在堆上分配值
fn box_demo() {
    println!("--- Box<T> Demo ---");
    // `b` 是一个指向在堆上分配的 `i32` 值的 `Box`。
    let b = Box::new(5);
    println!("b = {}", b);
    // 当 `b` 离开作用域时，`Box` 和它指向的数据都会被释放。
}

// 2. `Rc<T>` - 引用计数智能指针
#[allow(dead_code)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn rc_demo() {
    println!("\n--- Rc<T> Demo ---");
    // 创建一个列表 `a`，它被 `b` 和 `c` 共享。
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    // `b` 和 `c` 都持有对 `a` 的引用。
    let _b = List::Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = List::Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}

// 3. `RefCell<T>` - 内部可变性
pub trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger { sent_messages: RefCell::new(vec![]) }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

fn refcell_demo() {
    println!("\n--- RefCell<T> Demo ---");
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    println!("Messages sent: {:?}", mock_messenger.sent_messages.borrow());
}

fn main() {
    println!("Rust 智能指针演示\n");
    box_demo();
    rc_demo();
    refcell_demo();
    println!("\n演示完成。");
}