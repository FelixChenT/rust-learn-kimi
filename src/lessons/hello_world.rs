//! # Hello, world & Project Layout
//!
//! 目标：了解 Rust 项目基本结构和 Hello World 程序
//!
//! ## 要点
//! - `main()` 函数是程序入口点
//! - `println!` 是一个宏（macro），不是函数
//! - Rust 使用 `!` 表示宏调用
//! - 语句以分号 `;` 结尾
//!
//! ## 运行
//! `cargo run -- 01_hello_world`
//!
//! ## 测试
//! `cargo test -- --nocapture`

pub fn run() {
    println!("Hello, Rust learner! 🦀");
    println!("1 + 2 = {}", add(1, 2));

    let name = "Rust";
    println!("Welcome to {} programming!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-5, -3), -8);
    }
}