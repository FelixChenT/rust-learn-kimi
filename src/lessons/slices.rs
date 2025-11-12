//! # String & Array Slices
//!
//! 目标：理解 Rust 中的切片（slice）类型
//!
//! ## 要点
//! - 切片是对集合的部分引用，不拥有所有权
//! - 字符串切片：`&str` 是对 String 的部分引用
//! - 数组切片：`&[T]` 是对数组或向量的部分引用
//! - 切片是胖指针，包含指针和长度信息
//! - 字符串字面值是切片：`let s = "hello";` 类型是 `&str`
//!
//! ## 常见坑
//! - 切片索引越界会导致 panic
//! - 混淆 String 和 &str
//! - 忘记切片的生命周期不能超过原数据
//!
//! ## 运行
//! `cargo run -- 08_slices`

pub fn run() {
    println!("=== 字符串切片 ===");
    demo_string_slices();

    println!("\n=== 数组切片 ===");
    demo_array_slices();

    println!("\n=== 切片作为参数 ===");
    demo_slices_as_params();

    println!("\n=== 其他切片类型 ===");
    demo_other_slices();
}

fn demo_string_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // 或 &s[..5]
    let world = &s[6..11]; // 或 &s[6..]
    let whole = &s[..]; // 整个字符串

    println!("Original: '{}'", s);
    println!("hello: '{}'", hello);
    println!("world: '{}'", world);
    println!("whole: '{}'", whole);

    // 字符串字面值就是切片
    let literal: &str = "hello";
    println!("String literal: '{}'", literal);
}

fn demo_array_slices() {
    let arr = [1, 2, 3, 4, 5];

    let slice1 = &arr[1..3];
    let slice2 = &arr[..3];
    let slice3 = &arr[2..];
    let slice4 = &arr[..];

    println!("Original array: {:?}", arr);
    println!("arr[1..3]: {:?}", slice1);
    println!("arr[..3]: {:?}", slice2);
    println!("arr[2..]: {:?}", slice3);
    println!("arr[..]: {:?}", slice4);
}

fn demo_slices_as_params() {
    let s = String::from("Rust Programming");
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("First word of '{}' is '{}'", s, first_word(&s));
    println!("First 3 elements: {:?}", first_n(&arr, 3));
    println!("Second half: {:?}", second_half(&arr));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_n(arr: &[i32], n: usize) -> &[i32] {
    &arr[..n.min(arr.len())]
}

fn second_half(arr: &[i32]) -> &[i32] {
    &arr[arr.len() / 2..]
}

fn demo_other_slices() {
    let mut v = vec![1, 2, 3, 4, 5];

    // 切片可以修改原数据
    modify_first_two(&mut v);
    println!("After modification: {:?}", v);

    // 范围包含和排除
    println!("Inclusive range 1..=3 from {:?}: {:?}", v, &v[1..=3]);
    println!("Exclusive range 1..4 from {:?}: {:?}", v, &v[1..4]);
}

fn modify_first_two(slice: &mut [i32]) {
    if slice.len() >= 2 {
        slice[0] = 100;
        slice[1] = 200;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_slice() {
        let s = String::from("hello");
        let slice = &s[1..3];
        assert_eq!(slice, "el");
    }

    #[test]
    fn test_array_slice() {
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[1..4];
        assert_eq!(slice, [2, 3, 4]);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_first_n() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(first_n(&arr, 3), [1, 2, 3]);
        assert_eq!(first_n(&arr, 10), [1, 2, 3, 4, 5]);
        assert_eq!(first_n(&arr, 0), []);
    }

    #[test]
    fn test_modify_slice() {
        let mut arr = vec![1, 2, 3, 4, 5];
        modify_first_two(&mut arr);
        assert_eq!(arr[0], 100);
        assert_eq!(arr[1], 200);
        assert_eq!(arr[2], 3);
    }
}
