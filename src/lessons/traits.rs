//! # Traits & Trait Bounds
//!
//! 目标：理解 Rust 中的 trait 和 trait bounds
//!
//! ## 要点
//! - Trait 定义共享行为，类似于其他语言的接口
//! - Trait 可以包含方法签名、默认实现、关联类型
//! - 使用 `impl Trait for Type` 为类型实现 trait
//! - Trait bounds 限制泛型参数必须实现特定 trait
//! - 可以使用 `+` 指定多个 trait bounds
//!
//! ## 常见坑
//! - 忘记实现 trait 中的所有必需方法
//! - trait 对象的动态分发有性能开销
//! - trait 和类型必须在同一个 crate 中（孤儿规则）
//!
//! ## 运行
//! `cargo run -- 13_traits`

use std::fmt;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

pub trait Display {
    fn display(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn display(&self) -> String {
        format!("Tweet by {}: {}", self.username, self.content)
    }
}

struct Point {
    x: f64,
    y: f64,
}

trait Drawable {
    fn draw(&self);
}

trait Movable {
    fn move_to(&mut self, x: f64, y: f64);
}

impl Drawable for Point {
    fn draw(&self) {
        println!("Drawing point at ({}, {})", self.x, self.y);
    }
}

impl Movable for Point {
    fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

pub fn run() {
    println!("=== Trait 实现 ===");
    demo_trait_implementation();

    println!("");
    println!("=== 默认实现 ===");
    demo_default_implementation();

    println!("");
    println!("=== Trait Bounds ===");
    demo_trait_bounds();

    println!("");
    println!("=== 多个 Trait Bounds ===");
    demo_multiple_bounds();

    println!("");
    println!("=== Trait 作为参数 ===");
    demo_trait_as_param();
}

fn demo_trait_implementation() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };

    println!("New article available! {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());
}

fn demo_default_implementation() {
    struct BlogPost {
        title: String,
        author: String,
    }

    impl Summary for BlogPost {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    let post = BlogPost {
        title: String::from("MyPost"),
        author: String::from("author"),
    };

    // 使用默认的 summarize 实现
    println!("Blog post summary: {}", post.summarize());
    println!("Author: {}", post.summarize_author());
}

fn demo_trait_bounds() {
    fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    let tweet = Tweet {
        username: String::from("news"),
        content: String::from("Big news!"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
}

fn demo_multiple_bounds() {
    fn notify_multiple<T: Summary + Display>(item: &T) {
        println!("Summary: {}", item.summarize());
        println!("Display: {}", item.display());
    }

    let tweet = Tweet {
        username: String::from("multi"),
        content: String::from("Multiple traits!"),
        reply: false,
        retweet: false,
    };

    notify_multiple(&tweet);
}

fn demo_trait_as_param() {
    let tweet = Tweet {
        username: String::from("trait"),
        content: String::from("Trait object!"),
        reply: false,
        retweet: false,
    };

    println!("Tweet summary: {}", tweet.summarize());

    fn print_summary(item: &impl Summary) {
        println!("Summary from function: {}", item.summarize());
    }

    print_summary(&tweet);
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    }
}

trait Debug {
    fn debug(&self) -> String;
}

struct Calculator;

impl Debug for Calculator {
    fn debug(&self) -> String {
        String::from("Debug")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_implementation() {
        let tweet = Tweet {
            username: String::from("test"),
            content: String::from("test"),
            reply: false,
            retweet: false,
        };
        let expected = format!("@{}", "test");
        assert_eq!(tweet.summarize_author(), expected);
    }

    #[test]
    fn test_trait_bounds() {
        fn test_notify<T: Summary>(item: &T) -> String {
            item.summarize()
        }

        let tweet = Tweet {
            username: String::from("test"),
            content: String::from("content"),
            reply: false,
            retweet: false,
        };

        let summary = test_notify(&tweet);
        assert!(summary.contains("test"));
    }

    #[test]
    fn test_default_implementation() {
        struct TestItem;
        impl Summary for TestItem {
            fn summarize_author(&self) -> String {
                String::from("author")
            }
        }
        let item = TestItem;
        let expected = "(Read more...)";
        assert_eq!(item.summarize(), expected);
    }
}
