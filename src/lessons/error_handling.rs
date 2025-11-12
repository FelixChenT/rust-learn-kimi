//! # Error Handling: Result / Option / ? operator
//!
//! 目标：掌握 Rust 的错误处理机制
//!
//! ## 要点
//! - Rust 使用 `Result<T, E>` 处理可恢复错误
//! - `Option<T>` 处理可能为空的值
//! - `?` 运算符简化错误传播
//! - 可以使用 `unwrap()`、`expect()` 处理错误，但可能 panic
//! - 自定义错误类型实现 `Error` trait
//!
//! ## 常见坑
//! - 在不应 panic 的地方使用 unwrap
//! - 忘记处理 Err 或 None 情况
//! - 错误类型转换不当
//!
//! ## 运行
//! `cargo run -- 17_error_handling`

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::{error, fmt};

pub fn run() {
    println!("=== Option 类型 ===");
    demo_option();

    println!("\n=== Result 类型 ===");
    demo_result();

    println!("\n=== ? 运算符 ===");
    demo_question_operator();

    println!("\n=== 自定义错误类型 ===");
    demo_custom_error();
}

fn demo_option() {
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    let result = divide(10.0, 2.0);
    match result {
        Some(x) => println!("10 / 2 = {}", x),
        None => println!("Cannot divide by zero"),
    }

    if let Some(x) = divide(10.0, 0.0) {
        println!("Result: {}", x);
    } else {
        println!("Division failed");
    }

    let x = result.unwrap_or(0.0);
    println!("Unwrap or default: {}", x);
}

fn demo_result() {
    fn sqrt(x: f64) -> Result<f64, String> {
        if x >= 0.0 {
            Ok(x.sqrt())
        } else {
            Err(String::from("Cannot take square root of negative number"))
        }
    }

    match sqrt(16.0) {
        Ok(result) => println!("sqrt(16) = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match sqrt(-4.0) {
        Ok(result) => println!("sqrt(-4) = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    let content = read_file_content("test.txt");
    match content {
        Ok(text) => println!("File content (first 50 chars): {}", &text[..text.len().min(50)]),
        Err(e) => println!("Failed to read file: {}", e),
    }
}

fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn demo_question_operator() {
    fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
        let num = s.parse::<i32>()?;
        Ok(num * 2)
    }

    match parse_and_double("42") {
        Ok(result) => println!("Double of 42: {}", result),
        Err(e) => println!("Parse error: {}", e),
    }

    match parse_and_double("not a number") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Parse error: {}", e),
    }

    fn read_and_parse() -> Result<i32, Box<dyn error::Error>> {
        // ? 可以用于不同类型的错误，自动转换
        let content = read_file_content("test.txt");
        match content {
            Ok(s) => {
                let num = s.trim().parse()?;
                Ok(num)
            }
            Err(_) => Ok(0),
        }
    }

    match read_and_parse() {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error: {}", e),
    }
}

#[derive(Debug)]
enum AppError {
    FileNotFound(String),
    ParseError(String),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::FileNotFound(path) => write!(f, "File not found: {}", path),
            AppError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl error::Error for AppError {}

fn demo_custom_error() {
    fn divide_and_validate(a: i32, b: i32) -> Result<i32, AppError> {
        if b == 0 {
            return Err(AppError::InvalidInput(String::from("Cannot divide by zero")));
        }
        Ok(a / b)
    }

    fn process_number(s: &str) -> Result<i32, AppError> {
        s.parse::<i32>()
            .map_err(|e| AppError::ParseError(e.to_string()))
            .and_then(|n| divide_and_validate(n, 2))
    }

    match process_number("42") {
        Ok(result) => println!("42 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match process_number("not a number") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide_and_validate(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    let result = divide_and_validate(100, 4);
    if let Ok(value) = result {
        println!("100 / 4 = {}", value);
    }

    let result = divide_and_validate(100, 0);
    if let Err(e) = result {
        println!("Error occurred: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_some() {
        let x = Some(5);
        assert_eq!(x.unwrap(), 5);
    }

    #[test]
    fn test_option_none() {
        let x: Option<i32> = None;
        assert_eq!(x.unwrap_or(0), 0);
    }

    #[test]
    fn test_result_ok() {
        let x: Result<i32, &str> = Ok(42);
        assert_eq!(x.unwrap(), 42);
    }

    #[test]
    fn test_result_err() {
        let x: Result<i32, &str> = Err("error");
        assert!(x.is_err());
    }

    #[test]
    fn test_custom_error_display() {
        let err = AppError::FileNotFound(String::from("test.txt"));
        let err_str = format!("{}", err);
        assert!(err_str.contains("File not found"));
    }
}
