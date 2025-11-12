//! # Borrowing & References
//!
//! 目标：理解 Rust 中的借用和引用机制
//!
//! ## 要点
//! - 借用允许在不获取所有权的情况下使用值
//! - `&` 创建不可变引用（可同时多个）
//! - `&mut` 创建可变引用（同一时间只能有一个）
//! - 核心规则：要么一个可变引用，要么多个不可变引用，不能同时存在
//! - 引用生命周期不能超过被引用值的生命周期
//!
//! ## 常见坑
//! - 在拥有不可变引用的同时尝试创建可变引用
//! - 悬空引用：引用了已经 drop 的值
//! - 忘记解引用 `*` 操作符
//!
//! ## 运行
//! `cargo run -- 07_borrowing`

pub fn run() {
    println!("=== 不可变引用 ===");
    demo_immutable_reference();

    println!("\n=== 可变引用 ===");
    demo_mutable_reference();

    println!("\n=== 多个不可变引用 ===");
    demo_multiple_references();

    println!("\n=== 引用作为函数参数 ===");
    demo_reference_parameters();
}

fn demo_immutable_reference() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);
    // s 仍然有效，因为我们只是借用
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s 离开作用域，但因为只是引用，不拥有所有权，所以不会 drop
}

fn demo_mutable_reference() {
    let mut s = String::from("hello");
    println!("Before: {}", s);
    change(&mut s);
    println!("After: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn demo_multiple_references() {
    let s = String::from("hello");

    // 可以同时有多个不可变引用
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("{}, {}, and {}", r1, r2, r3);
    // r1, r2, r3 不再使用后，才能创建可变引用

    // 注意：r1, r2, r3 的最后一次使用在这里，之后才能创建可变引用
}

fn demo_reference_parameters() {
    let mut arr = vec![1, 2, 3, 4, 5];

    print_array(&arr);
    add_element(&mut arr, 6);
    print_array(&arr);

    let first = get_first(&arr);
    println!("First element: {:?}", first);
}

fn print_array(arr: &Vec<i32>) {
    println!("Array: {:?}", arr);
}

fn add_element(arr: &mut Vec<i32>, value: i32) {
    arr.push(value);
}

fn get_first(arr: &Vec<i32>) -> Option<&i32> {
    arr.get(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_reference() {
        let s = String::from("test");
        let len = calculate_length(&s);
        assert_eq!(len, 4);
        assert_eq!(s, "test");
    }

    #[test]
    fn test_mutable_reference() {
        let mut s = String::from("hello");
        change(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_multiple_immutable_refs() {
        let s = String::from("value");
        let r1 = &s;
        let r2 = &s;
        let r3 = &s;
        assert_eq!(*r1, "value");
        assert_eq!(*r2, "value");
        assert_eq!(*r3, "value");
    }

    #[test]
    fn test_reference_with_array() {
        let arr = vec![1, 2, 3, 4, 5];
        print_array(&arr);
        assert_eq!(arr.len(), 5);
    }

    #[test]
    fn test_mutable_reference_with_array() {
        let mut arr = vec![1, 2, 3];
        add_element(&mut arr, 4);
        assert_eq!(arr, vec![1, 2, 3, 4]);
    }
}
