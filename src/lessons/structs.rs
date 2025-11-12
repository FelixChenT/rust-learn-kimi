//! # Structs & Update Syntax
//!
//! 目标：理解 Rust 中的结构体（struct）定义和使用
//!
//! ## 要点
//! - 结构体是自定义数据类型，可以组合多个值
//! - 三种结构体：命名字段、元组结构体、unit 结构体
//! - 字段初始化简写：变量名和字段名相同时可省略
//! - 结构体更新语法：从其他结构体创建新实例
//! - Debug trait：使用 `#[derive(Debug)]` 打印调试信息
//!
//! ## 常见坑
//! - 结构体实例默认不可变，需要 `mut` 才能修改字段
//! - 忘记 `#[derive(Debug)]` 导致无法打印
//! - 结构体更新语法会移动字段所有权
//!
//! ## 运行
//! `cargo run -- 09_structs`

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct AlwaysEqual;

pub fn run() {
    println!("=== 命名字段结构体 ===");
    demo_named_structs();

    println!("\n=== 元组结构体 ===");
    demo_tuple_structs();

    println!("\n=== Unit 结构体 ===");
    demo_unit_structs();

    println!("\n=== 结构体更新语法 ===");
    demo_struct_update();
}

fn demo_named_structs() {
    let mut user = User {
        email: String::from("user@example.com"),
        username: String::from("rustacean"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {:#?}", user);

    user.email = String::from("newemail@example.com");
    user.sign_in_count += 1;

    println!("Updated user: {:#?}", user);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangle: {:?}", rect);
    println!("Rectangle area: {}", rect.width * rect.height);
}

fn demo_tuple_structs() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    println!("Black: {:?}", black);
    println!("White: {:?}", white);
    println!("Red component of black: {}", black.0);
}

fn demo_unit_structs() {
    let subject = AlwaysEqual;
    println!("Unit struct: {:?}", subject);
}

fn demo_struct_update() {
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 5,
    };

    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1 // 使用 user1 的 active 和 sign_in_count
    };

    println!("User1: {:#?}", user1);
    println!("User2: {:#?}", user2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        ..rect1 // 使用 rect1 的 height
    };

    println!("Rect1: {:?}", rect1);
    println!("Rect2: {:?}", rect2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            email: String::from("test@example.com"),
            username: String::from("testuser"),
            active: true,
            sign_in_count: 1,
        };
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
        assert!(user.active);
    }

    #[test]
    fn test_user_mutation() {
        let mut user = User {
            email: String::from("test@example.com"),
            username: String::from("test"),
            active: true,
            sign_in_count: 1,
        };
        user.sign_in_count = 2;
        assert_eq!(user.sign_in_count, 2);
    }

    #[test]
    fn test_struct_update() {
        let user1 = User {
            email: String::from("user1@example.com"),
            username: String::from("user1"),
            active: true,
            sign_in_count: 5,
        };

        let user2 = User {
            email: String::from("user2@example.com"),
            username: String::from("user2"),
            ..user1
        };

        assert_eq!(user2.active, user1.active);
        assert_eq!(user2.sign_in_count, user1.sign_in_count);
    }

    #[test]
    fn test_tuple_struct() {
        let color = Color(255, 128, 0);
        assert_eq!(color.0, 255);
        assert_eq!(color.1, 128);
        assert_eq!(color.2, 0);
    }
}
