//! # Variables & Mutability
//!
//! 目标：理解 Rust 中的变量、可变性、常量、遮蔽（shadowing）
//!
//! ## 要点
//! - 默认情况下变量是不可变的（immutable）
//! - 使用 `mut` 关键字使变量可变
//! - 常量（`const`）总是不可变，必须标注类型
//! - 变量可以遮蔽（shadow）之前的同名变量
//! - 遮蔽可以更改类型和可变性
//!
//! ## 常见坑
//! - 忘记 `mut` 导致编译错误
//! - 混淆变量遮蔽和可变赋值
//! - 常量命名没有使用大写加下划线
//!
//! ## 运行
//! `cargo run -- 02_variables`

pub fn run() {
    // 不可变变量
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // 这会导致编译错误！

    // 可变变量
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("Maximum points: {}", MAX_POINTS);

    // 变量遮蔽
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    // 类型转换遮蔽
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    demo_mutability();
}

fn demo_mutability() {
    let mut count = 0;
    println!("Initial count: {}", count);

    count += 1;
    println!("After increment: {}", count);

    // 遮蔽可以改变类型
    let count = "finished";
    println!("Count is now: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutability() {
        let mut value = 10;
        assert_eq!(value, 10);

        value = 20;
        assert_eq!(value, 20);
    }

    #[test]
    fn test_shadowing() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        assert_eq!(x, 12);
    }

    #[test]
    fn test_constants() {
        const TEST_VALUE: i32 = 42;
        assert_eq!(TEST_VALUE, 42);
    }
}