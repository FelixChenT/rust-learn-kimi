# Rust 基础语法学习项目

这是一个循序渐进的 Rust 基础语法学习项目，通过独立的 lesson 文件帮助初学者掌握 Rust 核心概念。

## 项目结构

每个 lesson 都是一个独立的 `.rs` 文件，包含：
- 清晰的模块注释（doc comments）
- 要点说明与示例
- 可运行的 `run()` 入口函数
- 至少 1 个对应的单元测试

## 目录结构

```
rust-learn-kimi/
├── Cargo.toml
├── rust-toolchain.toml     # 固定 Rust 版本
├── LICENSE                 # MIT 许可证
├── README.md              # 项目说明
├── .gitignore
├── src/
│   ├── main.rs            # CLI 入口
│   ├── lessons/           # 所有 lesson 模块
│   │   ├── mod.rs         # lesson 注册器
│   │   ├── 01_hello_world.rs
│   │   ├── 02_variables.rs
│   │   ├── 03_types.rs
│   │   └── ...（更多 lessons）
│   └── utils/             # 工具模块
└── tests/                 # 集成测试
```

## 快速开始

### 环境要求

- Rust 稳定版（stable channel）
- Cargo 包管理器

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 运行项目

列出所有 lessons：
```bash
cargo run -- list
```

运行指定 lesson（支持数字编号或 slug）：
```bash
cargo run -- 01_hello_world
cargo run -- 1  # 运行第1个 lesson
```

### 运行测试

```bash
cargo test
```

### 代码质量检查

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
```

## Lessons 列表

| 编号 | 文件名 | 主题 | 内容概述 |
|------|--------|------|----------|
| 01 | `01_hello_world.rs` | Hello, world & Project Layout | Rust 项目基础和 Hello World |
| 02 | `02_variables.rs` | Variables & Mutability | 变量和可变性 |
| 03 | `03_types.rs` | Scalar & Compound Types | 标量和复合类型 |
| 04 | `04_functions.rs` | Functions & Parameters | 函数和参数 |
| 05 | `05_control_flow.rs` | if / loop / while / match | 控制流 |
| 06 | `06_ownership.rs` | Ownership Basics | 所有权基础 |
| 07 | `07_borrowing.rs` | Borrowing & References | 借用和引用 |
| 08 | `08_slices.rs` | String & Array Slices | 字符串和数组切片 |
| 09 | `09_structs.rs` | Structs & Update Syntax | 结构体 |
| 10 | `10_enums_matching.rs` | Enums & Pattern Matching | 枚举和模式匹配 |
| 11 | `11_methods_assoc_fn.rs` | Methods & Associated Fns | 方法和关联函数 |
| 12 | `12_generics.rs` | Generics | 泛型 |
| 13 | `13_traits.rs` | Traits & Trait Bounds | 特性和特性约束 |
| 14 | `14_lifetimes.rs` | Lifetimes Basics | 生命周期基础 |
| 15 | `15_collections.rs` | Vec / String / HashMap | 集合类型 |
| 16 | `16_iterators_closures.rs` | Iterators & Closures | 迭代器和闭包 |
| 17 | `17_error_handling.rs` | Result / Option / ? operator | 错误处理 |
| 18 | `18_modules_crates.rs` | Modules / Crates / Paths | 模块和包管理 |
| 19 | `19_macros_basics.rs` | Macros Basics | 宏基础 |

## 贡献指南

### 新增 Lesson

1. 在 `src/lessons/` 目录下创建新文件，命名格式：`XX_topic_name.rs`（XX 为两位数字编号）
2. 文件必须包含：
   - 模块注释（`//!`）说明主题要点
   - 可运行的 `pub fn run()` 函数
   - 至少一个单元测试
3. 在 `src/lessons/mod.rs` 中注册新 lesson
4. 确保通过 `cargo fmt` 和 `cargo clippy` 检查
5. 添加单元测试：`cargo test`

### Lesson 文件模板

```rust
//! # Topic Name
//! 目标：用简短要点解释本课主题；给出 1-2 个最小可运行示例；列常见坑。
//! - 要点1
//! - 要点2
//! - 常见坑：……
//!
//! 运行：`cargo run -- XX_topic_name`
//! 测试：`cargo test -- --nocapture`

pub fn run() {
    // 示例：尽量打印出可辨识输出，便于对比
    println!("[Topic] demo output: {}", demo(2, 3));
}

fn demo(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(demo(2, 3), 5);
    }
}
```

## 开发规范

- 使用 `rustfmt` 进行代码格式化
- 使用 `clippy` 进行代码质量检查，必须无警告
- 每个 lesson 至少包含 1 个单元测试
- 示例代码优先最小可复现（MRE）
- 不同 lesson 间尽量零耦合

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 作者

FelixChenT - [GitHub](https://github.com/FelixChenT)

---

*Happy Rust learning! 🦀*