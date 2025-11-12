//! # Collections: Vec / String / HashMap
//!
//! 目标：理解 Rust 中常用的集合类型
//!
//! ## 要点
//! - `Vec<T>`：动态数组，在堆上存储同类型元素
//! - `String`：可增长的 UTF-8 字符串
//! - `HashMap<K, V>`：键值对映射
//! - 使用 `push` 添加元素，`pop` 移除元素
//! - 迭代器：可以遍历集合中的元素
//!
//! ## 常见坑
//! - 索引 Vec 越界会导致 panic，使用 `get` 更安全
//! - String 的 `+` 运算符会转移所有权
//! - HashMap 的键必须实现 `Eq` 和 `Hash` trait
//!
//! ## 运行
//! `cargo run -- 15_collections`

use std::collections::HashMap;

pub fn run() {
    println!("=== Vec 向量 ===");
    demo_vector();

    println!("\n=== String 字符串 ===");
    demo_string();

    println!("\n=== HashMap 哈希映射 ===");
    demo_hashmap();

    println!("\n=== 集合操作 ===");
    demo_collection_ops();
}

fn demo_vector() {
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("Vector v1: {:?}", v1);

    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("Vector v2: {:?}", v2);

    let third = &v2[2];
    println!("Third element: {}", third);

    match v2.get(10) {
        Some(value) => println!("Element at index 10: {}", value),
        None => println!("No element at index 10"),
    }

    for i in &mut v2 {
        *i += 50;
    }
    println!("Modified v2: {:?}", v2);

    v2.pop();
    println!("After pop: {:?}", v2);
}

fn demo_string() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: {}", s1);

    let mut s2 = String::from("Rust");
    s2.push(' ');
    s2.push_str("Programming");
    println!("s2: {}", s2);

    let s3 = String::from("Hello, ") + &s2;
    println!("s3: {}", s3);

    let s4 = format!("{} {}!", s1, s2);
    println!("s4: {}", s4);

    for c in s4.chars() {
        print!("[{}]", c);
    }
    println!();

    for b in s4.bytes() {
        print!("{}", b);
    }
    println!();
}

fn demo_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team score: {}", score);

    scores.insert(String::from("Blue"), 25);
    println!("Updated scores: {:?}", scores);

    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(100);
    println!("After entry: {:?}", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word count: {:?}", word_count);
}

fn demo_collection_ops() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.retain(|&x| x % 2 == 0);
    println!("Even numbers: {:?}", numbers);

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let sum: i32 = doubled.iter().sum();
    println!("Sum: {}", sum);

    let filtered: Vec<_> = (1..10).filter(|x| x % 3 == 0).collect();
    println!("Multiples of 3: {:?}", filtered);

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    if let Some(value) = map.get("key1") {
        println!("Found: {}", value);
    }

    map.remove("key1");
    println!("After removal: {:?}", map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_push_pop() {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_vector_get() {
        let v = vec![1, 2, 3];
        assert_eq!(v.get(1), Some(&2));
        assert_eq!(v.get(10), None);
    }

    #[test]
    fn test_string_concat() {
        let mut s = String::from("Hello");
        s.push_str(" World");
        assert_eq!(s, "Hello World");
    }

    #[test]
    fn test_hashmap_insert_get() {
        let mut map = HashMap::new();
        map.insert("key", 42);
        assert_eq!(map.get("key"), Some(&42));
    }

    #[test]
    fn test_hashmap_or_insert() {
        let mut map = HashMap::new();
        map.insert("a", 1);

        let count = map.entry("a").or_insert(0);
        *count += 1;
        assert_eq!(map.get("a"), Some(&2));

        let count = map.entry("b").or_insert(0);
        *count += 1;
        assert_eq!(map.get("b"), Some(&1));
    }

    #[test]
    fn test_collect_into_vec() {
        let v: Vec<i32> = (0..5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }
}
