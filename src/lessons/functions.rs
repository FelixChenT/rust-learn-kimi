//! # Functions & Parameters
//!
//! 目标：理解 Rust 中的函数定义、参数、返回值、语句与表达式
//!
//! ## 要点
//! - 函数定义使用 `fn` 关键字
//! - 参数必须声明类型
//! - 函数体由一系列语句组成，最后一个是可选的表达式作为返回值
//! - 使用 `return` 或隐式返回（不加分号）
//! - 多个参数使用逗号分隔
//!
//! ## 常见坑
//! - 误以为有分号的语句也能返回值
//! - 忘记在参数后标注类型
//! - 混淆语句（statement）和表达式（expression）
//!
//! ## 运行
//! `cargo run -- 04_functions`

pub fn run() {
    println!("=== 函数基础 ===");
    greet("Rust");
    greet("World");

    let sum = add(5, 10);
    println!("5 + 10 = {}", sum);

    println!("5 * 3 = {}", multiply(5, 3));

    println!("2^3 = {}", power(2, 3));

    println!("\n=== 无返回值函数 ===");
    print_message("Hello from function!");

    println!("\n=== 多参数函数 ===");
    println!("Area of 5x3 rectangle: {}", rectangle_area(5, 3));
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // 隐式返回（最后表达式无分号）
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y; // 显式返回
}

fn power(base: i32, exp: u32) -> i32 {
    let mut result = 1;
    for _ in 0..exp {
        result *= base;
    }
    result
}

fn print_message(msg: &str) {
    println!("Message: {}", msg);
    // 没有返回值，默认返回 ()
}

fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(0, 100), 0);
        assert_eq!(multiply(-2, 5), -10);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2, 3), 8);
        assert_eq!(power(5, 2), 25);
        assert_eq!(power(10, 0), 1);
        assert_eq!(power(2, 1), 2);
    }

    #[test]
    fn test_rectangle_area() {
        assert_eq!(rectangle_area(5, 3), 15);
        assert_eq!(rectangle_area(10, 10), 100);
        assert_eq!(rectangle_area(0, 5), 0);
    }
}
