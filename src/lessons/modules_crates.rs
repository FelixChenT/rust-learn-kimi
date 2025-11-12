//! # Modules / Crates / Paths
//!
//! 目标：理解 Rust 的模块系统和包管理
//!
//! ## 要点
//! - 模块（module）：组织代码、控制可见性
//! - 包（package）：包含一个或多个 crate
//! - Crate：库或可执行文件
//! - 路径（path）：引用模块项的方式
//! - `pub` 关键字控制可见性
//! - `use` 关键字引入路径
//! - `mod` 关键字声明模块
//!
//! ## 常见坑
//! - 忘记使用 `pub` 导致无法访问
//! - 混淆模块声明和模块文件
//! - 循环依赖问题
//!
//! ## 运行
//! `cargo run -- 18_modules_crates`

// 本章在单文件中演示模块系统概念

pub fn run() {
    println!("=== 模块基础 ===");
    demo_module_basics();

    println!("\n=== 路径和 use ===");
    demo_paths_and_use();

    println!("\n=== 可见性控制 ===");
    demo_visibility();

    println!("\n=== 嵌套模块 ===");
    demo_nested_modules();
}

fn demo_module_basics() {
    mod network {
        pub fn connect() {
            println!("Connecting to network...");
        }

        pub mod server {
            pub fn start() {
                println!("Server starting...");
                super::connect();
            }
        }
    }

    network::connect();
    network::server::start();

    mod config {
        pub struct Database {
            pub host: String,
            pub port: u32,
        }

        impl Database {
            pub fn new(host: String, port: u32) -> Self {
                Database { host, port }
            }

            pub fn connect(&self) {
                println!("Connecting to {}:{}", self.host, self.port);
            }
        }
    }

    let db = config::Database::new(String::from("localhost"), 3306);
    db.connect();
}

fn demo_paths_and_use() {
    mod math {
        pub const PI: f64 = 3.14159;

        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        pub mod advanced {
            pub fn power(base: i32, exp: u32) -> i32 {
                base.pow(exp)
            }
        }
    }

    println!("PI = {}", math::PI);
    println!("2 + 3 = {}", math::add(2, 3));
    println!("4 * 5 = {}", math::multiply(4, 5));
    println!("2^3 = {}", math::advanced::power(2, 3));

    use math::add;
    use math::advanced::power;

    println!("Using add directly: {}", add(10, 20));
    println!("Using power directly: {}", power(3, 2));

    use math::{multiply, PI};
    println!("Using multiply: {}", multiply(5, 6));
    println!("Using PI: {}", PI);

    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("HashMap: {:?}", map);

    use std::io::{self, Read};
    println!("Imported io and Read");

    use std::fmt::Result;
    use std::io::Result as IoResult;
    // Now we can use both Result types
}

fn demo_visibility() {
    mod backend {
        pub struct ApiClient {
            base_url: String,
        }

        impl ApiClient {
            pub fn new(base_url: String) -> Self {
                ApiClient { base_url }
            }

            pub fn get(&self, endpoint: &str) -> String {
                format!("GET {}/{}", self.base_url, endpoint)
            }

            fn log(&self, message: &str) {
                println!("[LOG] {}", message);
            }
        }
    }

    let client = backend::ApiClient::new(String::from("https://api.example.com"));
    println!("{}", client.get("users"));

    mod internal {
        pub fn public_api() {
            println!("Public API called");
            private_helper();
        }

        fn private_helper() {
            println!("Private helper");
        }
    }

    internal::public_api();
}

fn demo_nested_modules() {
    mod company {
        pub mod sales {
            pub fn generate_report() {
                println!("Sales report generated");
            }
        }

        pub mod engineering {
            pub fn deploy() {
                println!("Deployment started");
                super::sales::generate_report();
            }

            pub mod backend {
                pub fn migrate_database() {
                    println!("Database migration started");
                }
            }

            pub mod frontend {
                pub fn build_ui() {
                    println!("UI build started");
                    super::backend::migrate_database();
                }
            }
        }
    }

    company::sales::generate_report();
    company::engineering::deploy();
    company::engineering::backend::migrate_database();
    company::engineering::frontend::build_ui();

    mod utils {
        pub mod math {
            pub fn average(numbers: &[i32]) -> f64 {
                if numbers.is_empty() {
                    return 0.0;
                }
                let sum: i32 = numbers.iter().sum();
                sum as f64 / numbers.len() as f64
            }
        }

        pub mod string {
            pub fn is_palindrome(s: &str) -> bool {
                let cleaned: String = s.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase()).collect();
                cleaned == cleaned.chars().rev().collect::<String>()
            }
        }
    }

    let numbers = vec![1, 2, 3, 4, 5];
    println!("Average: {}", utils::math::average(&numbers));
    println!("Is palindrome 'racecar': {}", utils::string::is_palindrome("racecar"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_modules() {
        fn test_fn() {
            mod test_mod {
                pub fn test() -> i32 {
                    42
                }
            }
            assert_eq!(test_mod::test(), 42);
        }
        test_fn();
    }

    #[test]
    fn test_visibility() {
        mod test_visibility {
            pub fn public() -> i32 {
                private()
            }

            fn private() -> i32 {
                100
            }
        }
        assert_eq!(test_visibility::public(), 100);
    }
}
