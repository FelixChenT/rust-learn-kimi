//! # Macros Basics
//!
//! 目标：理解 Rust 中宏的基本概念和使用
//!
//! ## 要点
//! - 宏（Macro）：代码生成的元编程工具
//! - 声明式宏：`macro_rules!`
//! - 过程宏：自定义派生、属性宏、函数宏
//! - 宏在编译时展开，有 hygiene 特性
//! - 标准库常用宏：`println!`、`vec!`、`assert!` 等
//!
//! ## 常见坑
//! - 宏调试困难，错误信息不友好
//! - 宏可能导致代码膨胀
//! - 宏的 hygiene 规则复杂
//!
//! ## 运行
//! `cargo run -- 19_macros_basics`

pub fn run() {
    println!("=== 内置宏 ===");
    demo_builtin_macros();

    println!("\n=== 声明式宏 ===");
    demo_declarative_macros();

    println!("\n=== 自定义宏 ===");
    demo_custom_macros();

    println!("\n=== 宏的模式匹配 ===");
    demo_macro_pattern_matching();
}

fn demo_builtin_macros() {
    println!("println! macro");
    println!("Formatted: {}", format!("Hello, {}!", "Rust"));
    println!("Debug: {:?}", vec![1, 2, 3]);

    let v = vec![1, 2, 3, 4, 5];
    println!("vec! macro: {:?}", v);

    assert_eq!(2 + 2, 4);
    println!("assert_eq! passed");

    assert!(10 > 5);
    println!("assert! passed");

    let x = 42;
    debug_assert_eq!(x, 42);
    println!("debug_assert! passed");

    let s = stringify!(hello world);
    println!("stringify!: {}", s);
}

// 声明式宏示例
macro_rules! create_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! find_min {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr),+) => {
        {
            let min_y = find_min!($($y),+);
            if $x < min_y {
                $x
            } else {
                min_y
            }
        }
    };
}

macro_rules! hashmap {
    ($( $key:expr => $val:expr ),* $(,)?) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

fn demo_declarative_macros() {
    let v = create_vec![1, 2, 3, 4, 5];
    println!("Created vec: {:?}", v);

    let min = find_min!(10, 5, 8, 3, 15);
    println!("Minimum of [10, 5, 8, 3, 15] is {}", min);

    let map = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("HashMap: {:?}", map);
}

macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (sub $a:expr, $b:expr) => {
        $a - $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    (div $a:expr, $b:expr) => {
        $a / $b
    };
}

macro_rules! repeat {
    ($val:expr; $count:expr) => {
        {
            let mut vec = Vec::new();
            for _ in 0..$count {
                vec.push($val);
            }
            vec
        }
    };
}

macro_rules! impl_display_for_struct {
    ($struct_name:ident) => {
        impl ::std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", stringify!($struct_name))
            }
        }
    };
}

fn demo_custom_macros() {
    let sum = calculate!(add 5, 3);
    println!("5 + 3 = {}", sum);

    let diff = calculate!(sub 10, 4);
    println!("10 - 4 = {}", diff);

    let product = calculate!(mul 6, 7);
    println!("6 * 7 = {}", product);

    let quotient = calculate!(div 20, 5);
    println!("20 / 5 = {}", quotient);

    let zeros = repeat!(0; 5);
    println!("Zeros: {:?}", zeros);

    let ones = repeat!(1; 3);
    println!("Ones: {:?}", ones);

    struct MyStruct;
    impl_display_for_struct!(MyStruct);
    println!("MyStruct: {}", MyStruct);
}

macro_rules! generic_vec {
    () => {
        {
            Vec::new()
        }
    };
    ($x:expr) => {
        {
            let mut vec = Vec::new();
            vec.push($x);
            vec
        }
    };
    ($($x:expr),+ $(,)?) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($x);
            )+
            vec
        }
    };
}

macro_rules! create_struct {
    ($name:ident { $($field:ident : $val:expr),* $(,)? }) => {
        {
            struct $name {
                $(
                    $field: ::std::string::String,
                )*
            }

            $name {
                $(
                    $field: $val.to_string(),
                )*
            }
        }
    };
}

fn demo_macro_pattern_matching() {
    let single = generic_vec!(42);
    println!("Single element vec: {:?}", single);

    let multiple = generic_vec!(1, 2, 3, 4, 5);
    println!("Multiple elements vec: {:?}", multiple);

    let empty: Vec<i32> = generic_vec!();
    println!("Empty vec: {:?}", empty);
}

// 使用标准库的宏示例
fn demo_std_macros() {
    let v = vec![1, 2, 3];
    assert_eq!(v, [1, 2, 3]);

    let s = format!("Formatted string: {}", 42);
    println!("{}", s);

    let dbg_vec = vec![1, 2, 3];
    println!("Debug output: {:?}", dbg_vec);
}

#[macro_export]
macro_rules! test_case {
    ($name:ident, $expected:expr, $actual:expr) => {
        #[test]
        fn $name() {
            assert_eq!($expected, $actual);
        }
    };
}

test_case!(test_2_plus_2, 4, 2 + 2);
test_case!(test_3_times_3, 9, 3 * 3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vec_macro() {
        let v = create_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_find_min_macro() {
        let min = find_min!(10, 5, 8, 3, 15);
        assert_eq!(min, 3);
    }

    #[test]
    fn test_calculate_macro() {
        assert_eq!(calculate!(add 2, 3), 5);
        assert_eq!(calculate!(mul 4, 5), 20);
    }

    #[test]
    fn test_repeat_macro() {
        let v = repeat!(42; 3);
        assert_eq!(v, vec![42, 42, 42]);
    }

    #[test]
    fn test_hashmap_macro() {
        let map = hashmap! {
            "a" => 1,
            "b" => 2,
        };
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }
}
