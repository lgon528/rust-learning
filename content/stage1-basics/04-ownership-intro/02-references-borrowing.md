# 4.2 引用和借用

引用（References）和借用（Borrowing）是Rust所有权系统的核心概念，它们允许你在不获取所有权的情况下使用值。这是Rust实现内存安全和零成本抽象的关键机制。通过引用和借用，你可以编写既安全又高效的代码，避免不必要的数据复制和移动。

## 🎯 学习目标

- 理解引用和借用的概念及其重要性
- 掌握不可变引用的创建和使用
- 学会使用可变引用进行数据修改
- 理解借用规则和生命周期
- 掌握引用的作用域和有效性
- 学会解决常见的借用检查错误
- 了解悬垂引用的概念和预防
- 掌握引用在函数中的使用模式

## 🔗 什么是引用？

引用就像是一个指向某个值的"别名"或"指针"，但它不拥有该值。当引用离开作用域时，它指向的值不会被释放，因为引用没有所有权。

### 基本引用概念

```rust
fn main() {
    println!("=== 引用基础概念 ===");
    
    // 创建一个值
    let s1 = String::from("hello");
    
    // 创建引用：使用 & 操作符
    let s1_ref = &s1;  // s1_ref是s1的引用
    
    // 两个都可以使用
    println!("原始值: {}", s1);
    println!("引用值: {}", s1_ref);
    
    // 引用不拥有值，所以原始值仍然有效
    println!("s1仍然有效: {}", s1);
    
    // 多个不可变引用是允许的
    let s1_ref2 = &s1;
    let s1_ref3 = &s1;
    
    println!("多个引用: {}, {}, {}", s1_ref, s1_ref2, s1_ref3);
    
    // 演示引用与所有权的区别
    demonstrate_reference_vs_ownership();
    
    // 演示引用的内存表示
    demonstrate_reference_memory();
}

fn demonstrate_reference_vs_ownership() {
    println!("\n=== 引用 vs 所有权 ===");
    
    let original = String::from("original data");
    
    // 所有权转移
    let moved = original;
    // println!("{}", original);  // 编译错误：original已被移动
    println!("移动后: {}", moved);
    
    // 重新创建数据进行引用演示
    let data = String::from("reference data");
    
    // 创建引用
    let data_ref = &data;
    
    // 两个都可以使用
    println!("原始数据: {}", data);
    println!("引用数据: {}", data_ref);
    
    // 可以创建多个引用
    let another_ref = &data;
    println!("另一个引用: {}", another_ref);
    
    // 原始数据仍然拥有值
    println!("数据仍然有效: {}", data);
}

fn demonstrate_reference_memory() {
    println!("\n=== 引用的内存表示 ===");
    
    let number = 42;
    let number_ref = &number;
    
    println!("值: {}", number);
    println!("引用的值: {}", number_ref);
    println!("值的地址: {:p}", &number);
    println!("引用指向的地址: {:p}", number_ref);
    println!("引用本身的地址: {:p}", &number_ref);
    
    // 字符串的引用
    let text = String::from("Hello, Rust!");
    let text_ref = &text;
    
    println!("\n字符串值: {}", text);
    println!("字符串引用: {}", text_ref);
    println!("字符串地址: {:p}", &text);
    println!("引用指向地址: {:p}", text_ref);
    
    // 向量的引用
    let vec = vec![1, 2, 3, 4, 5];
    let vec_ref = &vec;
    
    println!("\n向量: {:?}", vec);
    println!("向量引用: {:?}", vec_ref);
    println!("向量地址: {:p}", &vec);
    println!("引用指向地址: {:p}", vec_ref);
}
```

## 📖 不可变引用

不可变引用允许你读取值，但不能修改它。你可以同时拥有多个不可变引用。

### 创建和使用不可变引用

```rust
fn main() {
    println!("=== 不可变引用演示 ===");
    
    let data = String::from("immutable reference example");
    
    // 创建不可变引用
    let ref1 = &data;
    let ref2 = &data;
    let ref3 = &data;
    
    // 可以同时使用多个不可变引用
    println!("引用1: {}", ref1);
    println!("引用2: {}", ref2);
    println!("引用3: {}", ref3);
    println!("原始数据: {}", data);
    
    // 不可变引用可以传递给函数
    print_length(ref1);
    print_length(&data);  // 直接传递引用
    
    // 演示不可变引用的只读特性
    demonstrate_readonly_nature();
    
    // 演示不可变引用在集合中的使用
    demonstrate_immutable_refs_in_collections();
    
    // 演示不可变引用的作用域
    demonstrate_immutable_ref_scope();
}

fn print_length(s: &String) {
    println!("字符串长度: {}", s.len());
    // s.push_str("test");  // 编译错误：不能通过不可变引用修改
}

fn demonstrate_readonly_nature() {
    println!("\n=== 不可变引用的只读特性 ===");
    
    let mut original = String::from("original");
    let immutable_ref = &original;
    
    // 可以读取
    println!("通过引用读取: {}", immutable_ref);
    println!("字符串长度: {}", immutable_ref.len());
    println!("是否为空: {}", immutable_ref.is_empty());
    
    // 不能修改
    // immutable_ref.push_str(" modified");  // 编译错误
    // immutable_ref.clear();  // 编译错误
    
    // 但是原始变量在引用不再使用后可以修改
    println!("引用使用完毕");
    
    // 在这里引用不再被使用，所以可以修改原始变量
    original.push_str(" - modified directly");
    println!("直接修改原始变量: {}", original);
}

fn demonstrate_immutable_refs_in_collections() {
    println!("\n=== 集合中的不可变引用 ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 创建引用向量
    let number_refs: Vec<&i32> = numbers.iter().collect();
    
    println!("原始向量: {:?}", numbers);
    println!("引用向量: {:?}", number_refs);
    
    // 通过引用访问元素
    for (i, num_ref) in number_refs.iter().enumerate() {
        println!("索引 {}: 值 {}", i, num_ref);
    }
    
    // 字符串向量的引用
    let strings = vec![
        String::from("first"),
        String::from("second"),
        String::from("third"),
    ];
    
    let string_refs: Vec<&String> = strings.iter().collect();
    
    for (i, s_ref) in string_refs.iter().enumerate() {
        println!("字符串 {}: {} (长度: {})", i, s_ref, s_ref.len());
    }
    
    // 原始数据仍然可用
    println!("原始字符串向量: {:?}", strings);
}

fn demonstrate_immutable_ref_scope() {
    println!("\n=== 不可变引用的作用域 ===");
    
    let mut data = String::from("scoped data");
    
    {
        let ref1 = &data;
        let ref2 = &data;
        
        println!("内部作用域 - ref1: {}", ref1);
        println!("内部作用域 - ref2: {}", ref2);
        
        // 在这个作用域内，不能修改data
        // data.push_str(" modified");  // 编译错误
    }  // ref1和ref2在这里离开作用域
    
    // 现在可以修改data了
    data.push_str(" - modified after refs");
    println!("修改后的数据: {}", data);
    
    // 演示引用的生命周期
    let long_lived_ref;
    {
        let short_lived = String::from("short lived");
        // long_lived_ref = &short_lived;  // 编译错误：悬垂引用
    }
    
    // 正确的做法
    let persistent_data = String::from("persistent");
    long_lived_ref = &persistent_data;
    println!("长生命周期引用: {}", long_lived_ref);
}
```

## ✏️ 可变引用

可变引用允许你修改借用的值。但是，在同一时间只能有一个可变引用，并且不能同时存在可变引用和不可变引用。

### 创建和使用可变引用

```rust
fn main() {
    println!("=== 可变引用演示 ===");
    
    // 必须是可变变量才能创建可变引用
    let mut data = String::from("mutable reference example");
    
    // 创建可变引用
    let mutable_ref = &mut data;
    
    // 通过可变引用修改值
    mutable_ref.push_str(" - modified");
    println!("通过可变引用修改: {}", mutable_ref);
    
    // 注意：当存在可变引用时，不能使用原始变量
    // println!("{}", data);  // 编译错误：不能在可变引用存在时使用原始变量
    
    // 可变引用使用完毕后，可以再次使用原始变量
    println!("可变引用使用完毕");
    println!("原始变量: {}", data);
    
    // 演示可变引用的独占性
    demonstrate_mutable_ref_exclusivity();
    
    // 演示可变引用在函数中的使用
    demonstrate_mutable_refs_in_functions();
    
    // 演示可变引用的作用域规则
    demonstrate_mutable_ref_scope_rules();
}

fn demonstrate_mutable_ref_exclusivity() {
    println!("\n=== 可变引用的独占性 ===");
    
    let mut value = 42;
    
    // 只能有一个可变引用
    let mut_ref1 = &mut value;
    // let mut_ref2 = &mut value;  // 编译错误：不能同时有多个可变引用
    
    *mut_ref1 += 10;
    println!("通过可变引用修改: {}", mut_ref1);
    
    // mut_ref1使用完毕后，可以创建新的可变引用
    println!("第一个可变引用使用完毕");
    
    let mut_ref2 = &mut value;
    *mut_ref2 *= 2;
    println!("通过新的可变引用修改: {}", mut_ref2);
    
    // 演示不可变引用和可变引用不能共存
    println!("\n--- 不可变引用和可变引用的互斥性 ---");
    
    let mut text = String::from("hello");
    
    // 可以有多个不可变引用
    let immut_ref1 = &text;
    let immut_ref2 = &text;
    
    println!("不可变引用1: {}", immut_ref1);
    println!("不可变引用2: {}", immut_ref2);
    
    // 不可变引用使用完毕后，可以创建可变引用
    println!("不可变引用使用完毕");
    
    let mut_ref = &mut text;
    mut_ref.push_str(", world!");
    println!("可变引用: {}", mut_ref);
    
    // 不能同时存在不可变引用和可变引用
    /*
    let immut_ref3 = &text;
    let mut_ref2 = &mut text;  // 编译错误
    println!("{}, {}", immut_ref3, mut_ref2);
    */
}

fn demonstrate_mutable_refs_in_functions() {
    println!("\n=== 函数中的可变引用 ===");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("修改前: {:?}", numbers);
    
    // 传递可变引用给函数
    double_values(&mut numbers);
    println!("加倍后: {:?}", numbers);
    
    add_element(&mut numbers, 6);
    println!("添加元素后: {:?}", numbers);
    
    // 字符串的可变引用
    let mut message = String::from("Hello");
    println!("修改前: {}", message);
    
    append_exclamation(&mut message);
    println!("添加感叹号后: {}", message);
    
    make_uppercase(&mut message);
    println!("转大写后: {}", message);
    
    // 结构体的可变引用
    demonstrate_struct_mutable_refs();
}

fn double_values(vec: &mut Vec<i32>) {
    for item in vec.iter_mut() {
        *item *= 2;
    }
}

fn add_element(vec: &mut Vec<i32>, element: i32) {
    vec.push(element);
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn make_uppercase(s: &mut String) {
    *s = s.to_uppercase();
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    fn new(name: String, age: u32, email: String) -> Self {
        Person { name, age, email }
    }
    
    fn celebrate_birthday(&mut self) {
        self.age += 1;
        println!("{} 现在 {} 岁了！", self.name, self.age);
    }
    
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

fn demonstrate_struct_mutable_refs() {
    println!("\n--- 结构体的可变引用 ---");
    
    let mut person = Person::new(
        String::from("Alice"),
        25,
        String::from("alice@example.com"),
    );
    
    println!("初始状态: {:?}", person);
    
    // 通过可变引用修改结构体
    update_person_age(&mut person, 26);
    update_person_email(&mut person, String::from("alice.smith@example.com"));
    
    println!("更新后: {:?}", person);
    
    // 调用结构体的可变方法
    person.celebrate_birthday();
    println!("生日后: {:?}", person);
}

fn update_person_age(person: &mut Person, new_age: u32) {
    person.age = new_age;
}

fn update_person_email(person: &mut Person, new_email: String) {
    person.update_email(new_email);
}

fn demonstrate_mutable_ref_scope_rules() {
    println!("\n=== 可变引用的作用域规则 ===");
    
    let mut data = vec![1, 2, 3];
    
    // 规则1：可变引用的作用域从创建开始，到最后一次使用结束
    {
        let mut_ref = &mut data;
        mut_ref.push(4);
        println!("可变引用使用: {:?}", mut_ref);
    }  // mut_ref在这里离开作用域
    
    // 现在可以创建新的引用
    let immut_ref = &data;
    println!("不可变引用: {:?}", immut_ref);
    
    // 规则2：引用的生命周期不能超过被引用值的生命周期
    let outer_ref;
    {
        let inner_data = vec![10, 20, 30];
        // outer_ref = &inner_data;  // 编译错误：悬垂引用
    }
    
    // 正确的做法
    let persistent_data = vec![100, 200, 300];
    outer_ref = &persistent_data;
    println!("有效的外部引用: {:?}", outer_ref);
    
    // 规则3：NLL (Non-Lexical Lifetimes) - 引用的生命周期优化
    demonstrate_nll();
}

fn demonstrate_nll() {
    println!("\n--- 非词法生命周期 (NLL) ---");
    
    let mut data = String::from("NLL example");
    
    // 在旧版本的Rust中，这会编译错误
    // 但在新版本中，由于NLL，这是允许的
    let immut_ref = &data;
    println!("不可变引用: {}", immut_ref);
    // immut_ref在这里最后一次使用
    
    // 由于immut_ref不再使用，可以创建可变引用
    let mut_ref = &mut data;
    mut_ref.push_str(" - NLL works!");
    println!("可变引用: {}", mut_ref);
    
    // 这展示了Rust编译器的智能生命周期分析
    println!("NLL使得引用使用更加灵活");
}
```

## 📏 借用规则

Rust的借用检查器（Borrow Checker）确保以下规则得到遵守：

### 核心借用规则

```rust
fn main() {
    println!("=== 借用规则演示 ===");
    
    /*
    借用规则：
    1. 在任意给定时间，要么只能有一个可变引用，要么只能有任意数量的不可变引用
    2. 引用必须总是有效的（不能是悬垂引用）
    3. 数据的所有者在有活跃引用时不能移动或销毁数据
    */
    
    // 规则1演示：互斥性
    demonstrate_exclusivity_rule();
    
    // 规则2演示：有效性
    demonstrate_validity_rule();
    
    // 规则3演示：所有者限制
    demonstrate_owner_restrictions();
    
    // 借用检查器的工作原理
    demonstrate_borrow_checker_analysis();
}

fn demonstrate_exclusivity_rule() {
    println!("\n=== 规则1：引用的互斥性 ===");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    // ✅ 允许：多个不可变引用
    {
        let ref1 = &data;
        let ref2 = &data;
        let ref3 = &data;
        
        println!("多个不可变引用: {:?}, {:?}, {:?}", ref1, ref2, ref3);
    }  // 不可变引用离开作用域
    
    // ✅ 允许：单个可变引用
    {
        let mut_ref = &mut data;
        mut_ref.push(6);
        println!("单个可变引用: {:?}", mut_ref);
    }  // 可变引用离开作用域
    
    // ❌ 不允许：同时存在可变引用和不可变引用
    /*
    let immut_ref = &data;
    let mut_ref = &mut data;  // 编译错误
    println!("{:?}, {:?}", immut_ref, mut_ref);
    */
    
    // ❌ 不允许：多个可变引用
    /*
    let mut_ref1 = &mut data;
    let mut_ref2 = &mut data;  // 编译错误
    println!("{:?}, {:?}", mut_ref1, mut_ref2);
    */
    
    // ✅ 允许：顺序使用不同类型的引用
    let immut_ref = &data;
    println!("不可变引用: {:?}", immut_ref);
    // immut_ref最后一次使用
    
    let mut_ref = &mut data;
    mut_ref.push(7);
    println!("可变引用: {:?}", mut_ref);
    // mut_ref最后一次使用
    
    println!("最终数据: {:?}", data);
}

fn demonstrate_validity_rule() {
    println!("\n=== 规则2：引用的有效性 ===");
    
    // ✅ 有效引用：引用的生命周期不超过被引用值
    let valid_ref;
    {
        let data = String::from("valid data");
        // 在同一作用域内使用引用
        let temp_ref = &data;
        println!("临时引用: {}", temp_ref);
    }  // data和temp_ref都离开作用域
    
    // ❌ 悬垂引用：引用指向已释放的内存
    /*
    let dangling_ref;
    {
        let data = String::from("will be dropped");
        dangling_ref = &data;  // 编译错误：悬垂引用
    }  // data被释放，但dangling_ref试图指向它
    println!("{}", dangling_ref);  // 使用无效引用
    */
    
    // ✅ 正确做法：确保被引用值的生命周期足够长
    let long_lived_data = String::from("long lived");
    valid_ref = &long_lived_data;
    println!("有效引用: {}", valid_ref);
    
    // 演示函数返回引用的规则
    demonstrate_function_reference_rules();
}

fn demonstrate_function_reference_rules() {
    println!("\n--- 函数引用规则 ---");
    
    let text = String::from("function reference example");
    
    // ✅ 返回输入参数的引用
    let first_word = get_first_word(&text);
    println!("第一个单词: {}", first_word);
    
    // ✅ 返回较长生命周期的引用
    let longer = "long string";
    let shorter = "short";
    let result = get_longer_string(longer, shorter);
    println!("较长的字符串: {}", result);
    
    // ❌ 不能返回局部变量的引用
    // let invalid = create_dangling_reference();  // 编译错误
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn get_longer_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ❌ 这个函数会产生悬垂引用
/*
fn create_dangling_reference() -> &String {
    let s = String::from("dangling");
    &s  // 编译错误：返回局部变量的引用
}  // s在这里被释放
*/

// ✅ 正确的做法：返回拥有的值
fn create_owned_string() -> String {
    String::from("owned")
}

fn demonstrate_owner_restrictions() {
    println!("\n=== 规则3：所有者限制 ===");
    
    let mut data = vec![1, 2, 3];
    
    // 当存在活跃引用时，不能移动或修改所有者
    let data_ref = &data;
    
    // ❌ 不能移动数据
    // let moved_data = data;  // 编译错误：不能在借用时移动
    
    // ❌ 不能通过所有者修改数据（当存在不可变引用时）
    // data.push(4);  // 编译错误：不能在不可变借用时修改
    
    println!("通过引用访问: {:?}", data_ref);
    // data_ref最后一次使用
    
    // ✅ 引用不再活跃后，可以修改所有者
    data.push(4);
    println!("修改后的数据: {:?}", data);
    
    // 演示可变引用的限制
    demonstrate_mutable_borrow_restrictions(&mut data);
}

fn demonstrate_mutable_borrow_restrictions(data: &mut Vec<i32>) {
    println!("\n--- 可变借用的限制 ---");
    
    // 当函数持有可变引用时，调用者不能访问原始数据
    data.push(5);
    data.push(6);
    
    println!("通过可变引用修改: {:?}", data);
    
    // 可变引用允许完全控制数据
    data.clear();
    data.extend_from_slice(&[10, 20, 30]);
    
    println!("重新填充数据: {:?}", data);
}

fn demonstrate_borrow_checker_analysis() {
    println!("\n=== 借用检查器分析 ===");
    
    let mut data = String::from("borrow checker analysis");
    
    // 借用检查器跟踪每个引用的生命周期
    println!("阶段1: 创建数据");
    
    {
        let ref1 = &data;  // 不可变借用开始
        println!("阶段2: 不可变借用 - {}", ref1);
        
        let ref2 = &data;  // 另一个不可变借用
        println!("阶段3: 多个不可变借用 - {} 和 {}", ref1, ref2);
        
        // ref1和ref2的生命周期在这里结束
    }
    
    println!("阶段4: 不可变借用结束");
    
    {
        let mut_ref = &mut data;  // 可变借用开始
        mut_ref.push_str(" - analyzed");
        println!("阶段5: 可变借用 - {}", mut_ref);
        // mut_ref的生命周期在这里结束
    }
    
    println!("阶段6: 可变借用结束");
    println!("最终数据: {}", data);
    
    // 演示复杂的借用场景
    demonstrate_complex_borrowing_scenarios();
}

fn demonstrate_complex_borrowing_scenarios() {
    println!("\n--- 复杂借用场景 ---");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // 场景1：条件借用
    let condition = true;
    if condition {
        let temp_ref = &mut numbers;
        temp_ref.push(6);
        println!("条件借用: {:?}", temp_ref);
    }
    
    // 场景2：循环中的借用
    for i in 0..3 {
        let element_ref = &numbers[i];
        println!("循环借用 {}: {}", i, element_ref);
    }
    
    // 场景3：匹配表达式中的借用
    match numbers.len() {
        0 => println!("空向量"),
        1..=5 => {
            let slice_ref = &numbers[0..3];
            println!("小向量切片: {:?}", slice_ref);
        },
        _ => {
            let full_ref = &numbers;
            println!("大向量: {:?}", full_ref);
        }
    }
    
    println!("复杂场景完成: {:?}", numbers);
}
```

## ⚠️ 常见错误和解决方案

### 1. 同时存在可变和不可变引用

```rust
fn main() {
    println!("=== 常见错误1：混合引用类型 ===");
    
    let mut data = vec![1, 2, 3];
    
    // ❌ 错误示例
    /*
    let immut_ref = &data;
    let mut_ref = &mut data;  // 编译错误
    println!("{:?}, {:?}", immut_ref, mut_ref);
    */
    
    // ✅ 解决方案1：分离使用
    let immut_ref = &data;
    println!("不可变引用: {:?}", immut_ref);
    // immut_ref使用完毕
    
    let mut_ref = &mut data;
    mut_ref.push(4);
    println!("可变引用: {:?}", mut_ref);
    
    // ✅ 解决方案2：使用作用域分离
    {
        let temp_immut = &data;
        println!("临时不可变引用: {:?}", temp_immut);
    }
    
    {
        let temp_mut = &mut data;
        temp_mut.push(5);
        println!("临时可变引用: {:?}", temp_mut);
    }
    
    println!("最终数据: {:?}", data);
}
```

### 2. 悬垂引用

```rust
fn main() {
    println!("\n=== 常见错误2：悬垂引用 ===");
    
    // ❌ 错误示例：返回局部变量的引用
    /*
    fn create_string_ref() -> &String {
        let s = String::from("local");
        &s  // 编译错误：返回悬垂引用
    }
    */
    
    // ✅ 解决方案1：返回拥有的值
    fn create_owned_string() -> String {
        String::from("owned")
    }
    
    let owned = create_owned_string();
    println!("拥有的字符串: {}", owned);
    
    // ✅ 解决方案2：接受引用参数并返回引用
    fn get_first_char(s: &str) -> Option<char> {
        s.chars().next()
    }
    
    let text = "Hello";
    if let Some(first) = get_first_char(&text) {
        println!("第一个字符: {}", first);
    }
    
    // ✅ 解决方案3：使用生命周期参数
    fn get_longer<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    
    let str1 = "short";
    let str2 = "longer string";
    let result = get_longer(str1, str2);
    println!("较长的字符串: {}", result);
}
```

### 3. 在迭代时修改集合

```rust
fn main() {
    println!("\n=== 常见错误3：迭代时修改 ===");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // ❌ 错误示例：在迭代时修改
    /*
    for num in &numbers {
        if *num % 2 == 0 {
            numbers.push(*num * 2);  // 编译错误：在借用时修改
        }
    }
    */
    
    // ✅ 解决方案1：收集需要添加的元素
    let mut to_add = Vec::new();
    for num in &numbers {
        if *num % 2 == 0 {
            to_add.push(*num * 2);
        }
    }
    numbers.extend(to_add);
    println!("添加偶数的两倍后: {:?}", numbers);
    
    // ✅ 解决方案2：使用索引迭代
    let original_len = numbers.len();
    for i in 0..original_len {
        if numbers[i] % 3 == 0 {
            numbers.push(numbers[i] * 3);
        }
    }
    println!("添加3的倍数的三倍后: {:?}", numbers);
    
    // ✅ 解决方案3：使用 drain 或其他消费迭代器
    let mut filtered: Vec<i32> = numbers.drain(..).filter(|&x| x < 20).collect();
    println!("过滤后: {:?}", filtered);
    
    // ✅ 解决方案4：使用 iter_mut 进行就地修改
    for num in filtered.iter_mut() {
        *num *= 10;
    }
    println!("就地修改后: {:?}", filtered);
}
```

### 4. 函数参数的借用问题

```rust
fn main() {
    println!("\n=== 常见错误4：函数参数借用 ===");
    
    let mut data = String::from("function borrowing");
    
    // ❌ 错误示例：函数获取所有权但调用者还想使用
    /*
    fn process_string_bad(s: String) -> String {
        format!("{} - processed", s)
    }
    
    let result = process_string_bad(data);
    println!("{}", data);  // 编译错误：data已被移动
    */
    
    // ✅ 解决方案1：使用引用
    fn process_string_ref(s: &str) -> String {
        format!("{} - processed", s)
    }
    
    let result1 = process_string_ref(&data);
    println!("原始数据: {}", data);
    println!("处理结果: {}", result1);
    
    // ✅ 解决方案2：返回所有权
    fn process_string_return(s: String) -> String {
        format!("{} - processed", s)
    }
    
    let result2 = process_string_return(data);
    // data不再可用，但result2包含了处理后的数据
    println!("处理并返回: {}", result2);
    
    // ✅ 解决方案3：使用可变引用进行就地修改
    let mut new_data = String::from("mutable processing");
    
    fn modify_string_inplace(s: &mut String) {
        s.push_str(" - modified in place");
    }
    
    modify_string_inplace(&mut new_data);
    println!("就地修改: {}", new_data);
    
    // ✅ 解决方案4：使用克隆（当性能不是关键时）
    let original = String::from("clone example");
    
    fn process_string_clone(s: String) -> (String, String) {
        let processed = format!("{} - processed", s);
        (s, processed)  // 返回原始和处理后的版本
    }
    
    let (returned_original, processed) = process_string_clone(original.clone());
    println!("原始（克隆前）: {}", original);
    println!("返回的原始: {}", returned_original);
    println!("处理后: {}", processed);
}
```

## 🎯 最佳实践

### 1. 优先使用引用而非所有权转移

```rust
fn main() {
    println!("=== 最佳实践1：优先使用引用 ===");
    
    let data = vec![1, 2, 3, 4, 5];
    
    // ✅ 好的做法：使用引用
    fn calculate_sum(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }
    
    fn find_max(numbers: &[i32]) -> Option<i32> {
        numbers.iter().max().copied()
    }
    
    let sum = calculate_sum(&data);
    let max = find_max(&data);
    
    println!("数据: {:?}", data);  // 仍然可用
    println!("总和: {}", sum);
    println!("最大值: {:?}", max);
    
    // 可以多次使用同一数据
    let sum2 = calculate_sum(&data);
    println!("再次计算总和: {}", sum2);
}
```

### 2. 合理使用可变引用

```rust
fn main() {
    println!("\n=== 最佳实践2：合理使用可变引用 ===");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // ✅ 好的做法：明确的可变操作
    fn double_in_place(numbers: &mut Vec<i32>) {
        for num in numbers.iter_mut() {
            *num *= 2;
        }
    }
    
    fn add_element_if_needed(numbers: &mut Vec<i32>, threshold: usize) {
        if numbers.len() < threshold {
            numbers.push(0);
        }
    }
    
    println!("原始: {:?}", numbers);
    
    double_in_place(&mut numbers);
    println!("加倍后: {:?}", numbers);
    
    add_element_if_needed(&mut numbers, 10);
    println!("添加元素后: {:?}", numbers);
    
    // ✅ 限制可变引用的作用域
    {
        let mut_ref = &mut numbers;
        mut_ref.sort();
        mut_ref.reverse();
    }  // 可变引用在这里结束
    
    // 现在可以创建不可变引用
    let immut_ref = &numbers;
    println!("排序并反转后: {:?}", immut_ref);
}
```

### 3. 使用适当的字符串类型

```rust
fn main() {
    println!("\n=== 最佳实践3：字符串类型选择 ===");
    
    // ✅ 对于只读操作，使用 &str
    fn count_words(text: &str) -> usize {
        text.split_whitespace().count()
    }
    
    fn starts_with_vowel(word: &str) -> bool {
        word.chars().next()
            .map(|c| "aeiouAEIOU".contains(c))
            .unwrap_or(false)
    }
    
    // ✅ 对于需要修改的操作，使用 &mut String
    fn capitalize_words(text: &mut String) {
        *text = text.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
    }
    
    let text = "hello world rust programming";
    println!("单词数: {}", count_words(text));
    
    let words: Vec<&str> = text.split_whitespace().collect();
    for word in words {
        println!("{} 以元音开头: {}", word, starts_with_vowel(word));
    }
    
    let mut owned_text = text.to_string();
    println!("原始: {}", owned_text);
    
    capitalize_words(&mut owned_text);
    println!("首字母大写: {}", owned_text);
}
```

## ✅ 学习检查清单

- [ ] 理解引用和借用的基本概念
- [ ] 掌握不可变引用的创建和使用
- [ ] 掌握可变引用的创建和使用
- [ ] 理解借用规则的三个核心原则
- [ ] 能够识别和避免悬垂引用
- [ ] 理解引用的作用域和生命周期
- [ ] 掌握在函数中正确使用引用
- [ ] 能够解决常见的借用检查错误
- [ ] 了解NLL（非词法生命周期）的概念
- [ ] 掌握引用在集合和复杂数据结构中的使用

## 📖 扩展阅读

- [Rust官方文档 - 引用和借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust Reference - 借用](https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrow-operators)
- [Rust by Example - 借用](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)
- [借用检查器的工作原理](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes)

---

**下一节预告：** 在下一节中，我们将学习切片（Slices），了解如何引用集合中的连续元素序列。