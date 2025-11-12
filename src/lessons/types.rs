//! # Scalar & Compound Types
//!
//! 目标：理解 Rust 的基本数据类型，包括标量类型和复合类型
//!
//! ## 要点
//! ### 标量类型（Scalar Types）
//! - 整数类型：`i8`, `i16`, `i32`, `i64`, `i128`, `isize`（有符号）
//! - 整数类型：`u8`, `u16`, `u32`, `u64`, `u128`, `usize`（无符号）
//! - 浮点类型：`f32`, `f64`
//! - 布尔类型：`bool`
//! - 字符类型：`char`（Unicode 标量值）
//!
//! ### 复合类型（Compound Types）
//! - 元组（Tuple）：可以包含不同类型，固定长度
//! - 数组（Array）：相同类型，固定长度
//! - 向量（Vector）：相同类型，可变长度
//!
//! ## 常见坑
//! - 整数溢出（debug 模式会 panic）
//! - 数组越界访问（会 panic）
//! - 字符类型使用单引号，字符串使用双引号
//!
//! ## 运行
//! `cargo run -- 03_types`

pub fn run() {
    demo_scalar_types();
    demo_compound_types();
    demo_type_inference();
}

fn demo_scalar_types() {
    println!("=== 标量类型 ===");

    // 整数
    let x: i32 = 42;
    let y: u64 = 100_000;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;

    println!("整数: x={}, y={}, hex={}, octal={}, binary={}", x, y, hex, octal, binary);

    // 浮点数
    let f1: f32 = 3.14;
    let f2: f64 = 3.14159265359;
    println!("浮点数: f32={}, f64={}", f1, f2);

    // 布尔值
    let t = true;
    let f: bool = false;
    println!("布尔值: t={}, f={}", t, f);

    // 字符
    let c1 = 'A';
    let c2 = '🦀';
    let c3 = '中';
    println!("字符: c1='{}', c2='{}', c3='{}'", c1, c2, c3);
}

fn demo_compound_types() {
    println!("\n=== 复合类型 ===");

    // 元组
    let tuple: (i32, f64, char) = (42, 3.14, 'A');
    let (x, y, z) = tuple;
    println!("元组: tuple=({},{},{}), x={}, y={}, z={}", tuple.0, tuple.1, tuple.2, x, y, z);

    // 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let last = arr[4];
    println!("数组: arr={:?}, first={}, last={}", arr, first, last);

    // 向量
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    vec.push(5);
    println!("向量: vec={:?}, len={}, capacity={}", vec, vec.len(), vec.capacity());
}

fn demo_type_inference() {
    println!("\n=== 类型推断 ===");

    // Rust 可以自动推断类型
    let x = 42;          // i32
    let y = 3.14;        // f64
    let z = true;        // bool
    let s = "hello";     // &str

    println!("推断类型: x={}, y={}, z={}, s={}", x, y, z, s);

    // 显示指定类型
    let explicit: u8 = 255;
    println!("显式类型: explicit={}", explicit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_types() {
        let x: i32 = 42;
        let y: u64 = 100;
        assert_eq!(x, 42);
        assert_eq!(y, 100);
    }

    #[test]
    fn test_float_types() {
        let x: f32 = 3.14;
        let y: f64 = 2.718;
        assert!((x - 3.14).abs() < 0.001);
        assert!((y - 2.718).abs() < 0.001);
    }

    #[test]
    fn test_tuple() {
        let tuple = (1, 2.0, "three");
        assert_eq!(tuple.0, 1);
        assert_eq!(tuple.1, 2.0);
        assert_eq!(tuple.2, "three");
    }

    #[test]
    fn test_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(arr.len(), 5);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[4], 5);
    }

    #[test]
    fn test_vector() {
        let mut vec = vec![1, 2, 3];
        vec.push(4);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec[3], 4);
    }
}