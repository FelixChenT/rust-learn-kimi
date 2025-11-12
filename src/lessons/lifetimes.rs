//! # Lifetimes Basics
//!
//! 目标：理解 Rust 中的生命周期（lifetimes）
//!
//! ## 要点
//! - 生命周期是 Rust  borrow checker 的核心机制
//! - 防止悬空引用：确保引用始终有效
//! - 每个引用都有生命周期（隐式或显式）
//! - 函数签名中的生命周期参数：标注输入输出引用的关系
//! - 生命周期省略规则（lifetime elision）简化代码
//! - `'static` 生命周期：整个程序的生命周期
//!
//! ## 常见坑
//! - 返回值引用可能与多个输入引用相关，导致歧义
//! - 不理解生命周期省略规则
//! - 试图返回局部变量的引用
//!
//! ## 运行
//! `cargo run -- 14_lifetimes`

use std::fmt;

pub fn run() {
    println!("=== 生命周期基础 ===");
    demo_lifetime_basics();

    println!("\n=== 函数中的生命周期 ===");
    demo_function_lifetimes();

    println!("\n=== 结构体中的生命周期 ===");
    demo_struct_lifetimes();

    println!("\n=== 静态生命周期 ===");
    demo_static_lifetime();
}

fn demo_lifetime_basics() {
    let s1 = String::from("hello");
    {
        let s2 = String::from("world");
        let result = longest(&s1, &s2);
        println!("The longest string is '{}'", result);
    }
    // s2 在这里被 drop，但 result 只在内部作用域有效

    let s3 = String::from("rust");
    let result = longest(&s1, &s3);
    println!("The longest string is '{}'", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demo_function_lifetimes() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is '{}'", result);

    let result2 = first_word(&string1);
    println!("The first word is: {}", result2);
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn demo_struct_lifetimes() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {}", i.part);
    println!("Level: {}", i.level());
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn demo_static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
    println!("'static string: {}", s);

    let num: &'static i32 = &42;
    println!("'static number: {}", num);

    // 字符串字面值默认是 'static
    let s2 = "This is also 'static";
    println!("String literal: {}", s2);
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let s1 = String::from("hello");
        let s2 = String::from("world!");
        assert_eq!(longest(&s1, &s2), "world!");
    }

    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn test_important_excerpt() {
        let s = String::from("Important text here.");
        let excerpt = ImportantExcerpt { part: &s[0..9] };
        assert_eq!(excerpt.level(), 3);
        assert_eq!(excerpt.part, "Important");
    }

    #[test]
    fn test_static_lifetime() {
        let s = "Static string";
        assert_eq!(s.len(), 13);
    }
}
