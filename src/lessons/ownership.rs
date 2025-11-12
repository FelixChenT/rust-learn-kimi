//! # Ownership Basics
//!
//! 目标：理解 Rust 所有权的核心概念
//!
//! ## 要点
//! - Rust 的核心特性是所有权系统，无需垃圾回收即可保证内存安全
//! - 每个值都有唯一的所有者（owner）
//! - 当所有者超出作用域，值会被自动丢弃（drop）
//! - 所有权转移（move）：将值赋给另一个变量时，所有权发生转移
//! - 数据在栈上（如基本类型）会复制，在堆上（如 String）会移动
//!
//! ## 常见坑
//! - 移动后原变量不能再使用
//! - 不理解堆和栈的区别
//! - 忘记所有权规则会导致编译错误
//!
//! ## 运行
//! `cargo run -- 06_ownership`

pub fn run() {
    println!("=== 所有权基础 ===");
    demo_ownership_move();

    println!("\n=== 作用域与丢弃 ===");
    demo_scope_drop();

    println!("\n=== 栈 vs 堆 ===");
    demo_stack_heap();
}

fn demo_ownership_move() {
    // 基本类型（在栈上）：会复制（Copy trait）
    let x = 5;
    let y = x; // x 被复制到 y，x 仍然有效
    println!("Stack values: x={}, y={}", x, y);

    // String 类型（在堆上）：会发生移动（Move）
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动给 s2，s1 失效

    println!("Heap values: s2={}", s2);
    // println!("s1={}", s1); // 这会导致编译错误！

    // 克隆可以显式复制
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("Cloned: s3={}, s4={}", s3, s4);
}

fn demo_scope_drop() {
    {
        let s = String::from("inside scope");
        println!("In scope: {}", s);
    } // s 在这里被 drop

    println!("Out of scope (s was dropped)");

    // 函数参数也会发生所有权转移
    let s = String::from("give away");
    take_ownership(s); // s 的所有权移动到函数
    // println!("s={}", s); // 编译错误：s 已被移动

    // 基本类型实现了 Copy trait，不会移动
    let x = 42;
    make_copy(x);
    println!("x still works: {}", x);
}

fn take_ownership(s: String) {
    println!("I took ownership of: {}", s);
    // s 在函数结束时被 drop
}

fn make_copy(x: i32) {
    println!("I made a copy of: {}", x);
    // x 在函数结束时不会 drop，因为是 Copy
}

fn demo_stack_heap() {
    println!("Stack types (Copy trait):");
    let a = 10;
    let b = a;
    println!("  Integers: a={}, b={}", a, b);

    let c = true;
    let d = c;
    println!("  Booleans: c={}, d={}", c, d);

    println!("Heap types (Move semantics):");
    let s1 = String::from("hello");
    let s2 = s1;
    println!("  String: s2={}", s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_trait() {
        let x = 5;
        let y = x;
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_clone() {
        let s1 = String::from("test");
        let s2 = s1.clone();
        assert_eq!(s1, "test");
        assert_eq!(s2, "test");
    }

    #[test]
    fn test_function_ownership() {
        let s = String::from("hello");
        // s 被移动到函数中
        take_ownership(s);
        // s 不能再使用

        let x = 42;
        make_copy(x);
        // x 仍然可用
        assert_eq!(x, 42);
    }
}
