//! # Enums & Pattern Matching
//!
//! 目标：理解 Rust 中的枚举和模式匹配
//!
//! ## 要点
//! - 枚举允许定义一个类型可以是多个变体之一
//! - 每个变体可以包含不同类型的数据
//! - `match` 是模式匹配的主要方式，必须穷尽所有情况
//! - 可以使用 `Option<T>` 和 `Result<T, E>` 处理可能缺失或失败的情况
//! - `if let` 是 match 的简写，用于只关心一种模式的情况
//!
//! ## 常见坑
//! - match 必须包含所有情况或使用通配符 `_`
//! - 忘记处理 `Option::None` 或 `Result::Err`
//! - 模式匹配的顺序很重要
//!
//! ## 运行
//! `cargo run -- 10_enums_matching`

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn run() {
    println!("=== 基本枚举 ===");
    demo_basic_enums();

    println!("\n=== 带数据的枚举 ===");
    demo_enums_with_data();

    println!("\n=== Option 枚举 ===");
    demo_option_enum();

    println!("\n=== 模式匹配 ===");
    demo_pattern_matching();

    println!("\n=== 多分支匹配 ===");
    demo_multi_branch();
}

fn demo_basic_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);
}

fn demo_enums_with_data() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    let msg1 = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 10, y: 20 };

    println!("Message 1: {:?}", msg1);
    println!("Message 2: {:?}", msg2);
}

fn demo_option_enum() {
    let some_number = Some(5);
    let some_string = Some(String::from("hello"));
    let absent_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent: {:?}", absent_number);

    let x = 5;
    let y = some_number.unwrap_or(0);
    println!("x + y = {}", x + y);
}

fn demo_pattern_matching() {
    let msg = Message::ChangeColor(255, 128, 0);

    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }

    let some_value = Some(42);
    match some_value {
        Some(x) if x > 40 => println!("Large number: {}", x),
        Some(x) => println!("Number: {}", x),
        None => println!("No value"),
    }
}

fn demo_multi_branch() {
    let value = 42;

    let category = match value {
        0 => "zero",
        1..=10 => "small",
        11..=100 => "medium",
        101..=1000 => "large",
        _ => "huge",
    };
    println!("{} is {}", value, category);

    let ip = IpAddr::V4(192, 168, 1, 1);
    let kind = match ip {
        IpAddr::V4(_, _, _, _) => "IPv4",
        IpAddr::V6(_) => "IPv6",
    };
    println!("IP address is {}", kind);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_creation() {
        let kind = IpAddrKind::V4;
        assert!(matches!(kind, IpAddrKind::V4));
    }

    #[test]
    fn test_enum_with_data() {
        let ip = IpAddr::V4(127, 0, 0, 1);
        if let IpAddr::V4(a, b, c, d) = ip {
            assert_eq!(a, 127);
            assert_eq!(b, 0);
            assert_eq!(c, 0);
            assert_eq!(d, 1);
        }
    }

    #[test]
    fn test_option() {
        let some_value = Some(10);
        assert_eq!(some_value.unwrap(), 10);

        let none_value: Option<i32> = None;
        assert_eq!(none_value.unwrap_or(0), 0);
    }

    #[test]
    fn test_pattern_matching() {
        let msg = Message::Write(String::from("test"));
        if let Message::Write(text) = msg {
            assert_eq!(text, "test");
        } else {
            panic!("Pattern match failed");
        }
    }

    #[test]
    fn test_match_exhaustive() {
        let value = Some(42);
        let result = match value {
            Some(x) => x * 2,
            None => 0,
        };
        assert_eq!(result, 84);
    }
}
