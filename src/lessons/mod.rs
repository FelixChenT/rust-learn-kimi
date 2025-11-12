//! Lessons 模块注册器
//!
//! 统一管理所有 lesson 模块，提供 list 和运行功能

macro_rules! register_lessons {
    ($($num:literal, $slug:ident, $title:expr, $path:ident),+ $(,)?) => {
        $(
            pub mod $path;
        )+

        pub struct Lesson {
            pub number: usize,
            pub slug: &'static str,
            pub title: &'static str,
            pub run: fn(),
        }

        pub fn all() -> Vec<Lesson> {
            vec![
                $(
                    Lesson {
                        number: $num,
                        slug: stringify!($slug),
                        title: $title,
                        run: $path::run,
                    }
                ),+
            ]
        }

        pub fn list() {
            for l in all() {
                println!("{:02}  {:<24} {}", l.number, l.slug, l.title);
            }
        }

        pub fn run_selected(sel: &str) -> Result<(), String> {
            let lessons = all();
            // 支持数字或 slug
            if let Ok(n) = sel.parse::<usize>() {
                if let Some(l) = lessons.iter().find(|l| l.number == n) {
                    (l.run)();
                    return Ok(());
                }
            }
            if let Some(l) = lessons.iter().find(|l| l.slug == sel) {
                (l.run)();
                return Ok(());
            }
            Err(format!("Lesson '{}' not found", sel))
        }
    };
}

// —— 在这里登记全部 lesson ——
register_lessons!(
    1,
    hello_world,
    "Hello, world & Project Layout",
    hello_world,
    2,
    variables,
    "Variables & Mutability",
    variables,
    3,
    types,
    "Scalar & Compound Types",
    types,
    4,
    functions,
    "Functions & Parameters",
    functions,
    5,
    control_flow,
    "if / loop / while / match",
    control_flow,
    6,
    ownership,
    "Ownership Basics",
    ownership,
    7,
    borrowing,
    "Borrowing & References",
    borrowing,
    8,
    slices,
    "String & Array Slices",
    slices,
    9,
    structs,
    "Structs & Update Syntax",
    structs,
    10,
    enums_matching,
    "Enums & Pattern Matching",
    enums_matching,
    11,
    methods_assoc_fn,
    "Methods & Associated Fns",
    methods_assoc_fn,
    12,
    generics,
    "Generics",
    generics,
    13,
    traits,
    "Traits & Trait Bounds",
    traits,
    14,
    lifetimes,
    "Lifetimes Basics",
    lifetimes,
    15,
    collections,
    "Vec / String / HashMap",
    collections,
    16,
    iterators_closures,
    "Iterators & Closures",
    iterators_closures,
    17,
    error_handling,
    "Result / Option / ? operator",
    error_handling,
    18,
    modules_crates,
    "Modules / Crates / Paths",
    modules_crates,
    19,
    macros_basics,
    "Macros Basics",
    macros_basics,
);
