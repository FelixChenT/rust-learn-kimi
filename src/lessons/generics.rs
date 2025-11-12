//! # Generics
//!
//! 目标：理解 Rust 中的泛型编程
//!
//! ## 要点
//! - 泛型允许编写可重用的代码，适用于多种类型
//! - 在函数、结构体、枚举和方法中使用泛型
//! - 编译器通过单态化（monomorphization）为具体类型生成代码
//! - 使用 trait bounds 限制泛型类型可以实现的行为
//!
//! ## 常见坑
//! - 忘记添加必要的 trait bounds
//! - 泛型代码可能导致编译后体积增大
//! - 复杂的泛型签名难以阅读
//!
//! ## 运行
//! `cargo run -- 12_generics`

use std::cmp::PartialOrd;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
}

pub fn run() {
    println!("=== 泛型函数 ===");
    demo_generic_functions();

    println!("\n=== 泛型结构体 ===");
    demo_generic_structs();

    println!("\n=== 泛型方法 ===");
    demo_generic_methods();

    println!("\n=== 泛型枚举 ===");
    demo_generic_enums();

    println!("\n=== Trait Bounds ===");
    demo_trait_bounds();
}

fn demo_generic_functions() {
    println!("Largest integer in [1, 2, 3, 4, 5]: {}", largest(&[1, 2, 3, 4, 5]));
    println!("Largest char in ['a', 'b', 'c']: {}", largest(&['a', 'b', 'c']));

    let mut p1 = (3, 5);
    let p2 = (10, 20);
    println!("P1: {:?}, P2: {:?}", p1, p2);
    swap(&mut p1.0, &mut p1.1);
    println!("Swapped P1: {:?}", p1);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn swap<T>(x: &mut T, y: &mut T) {
    std::mem::swap(x, y);
}

fn demo_generic_structs() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let int_float_pair = Pair { first: 5, second: "hello" };

    println!("Integer point: {:?}", integer_point);
    println!("Float point: {:?}", float_point);
    println!("Int-Float pair: {:?}", int_float_pair);

    let string_int_pair = Pair::new(String::from("test"), 42);
    println!("String-Int pair: {:?}", string_int_pair);

    let tuple_pair = Pair::new((1, 2), (3, 4));
    println!("Tuple pair: {:?}", tuple_pair);
}

fn demo_generic_methods() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.5, y: 4.5 };

    println!("P1: x={}, y={}", p1.x(), p1.y());
    println!("P2: x={}, y={}", p2.x(), p2.y());

    let distance = p1.distance(&p2);
    println!("Distance from P1 to P2: {}", distance);
}

impl<T> Point<T>
where
    T: PartialOrd + Copy + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Into<f64>,
{
    fn distance<U>(&self, other: &Point<U>) -> f64
    where
        U: Copy + Into<f64>,
        T: Copy + Into<f64>,
    {
        let x1: f64 = self.x.into();
        let y1: f64 = self.y.into();
        let x2: f64 = other.x.into();
        let y2: f64 = other.y.into();
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
}

fn demo_generic_enums() {
    let some_number = Option::Some(5);
    let some_string = Option::Some(String::from("hello"));
    let absent_number: Option<i32> = Option::None;

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);

    let success: Result<i32, &str> = Result::Ok(42);
    let error: Result<i32, &str> = Result::Err("Something went wrong");

    println!("Success: {:?}", success);
    println!("Error: {:?}", error);
}

fn demo_trait_bounds() {
    let int_list = vec![1, 2, 3, 4, 5];
    let float_list = vec![1.1, 2.2, 3.3];

    println!("Sorted ints: {:?}", sort_desc(int_list));
    println!("Sorted floats: {:?}", sort_desc(float_list));

    let p1 = Point { x: 3, y: 5 };
    let p2 = Point { x: 10, y: 20 };
    println!("P1 < P2: {}", compare_points(&p1, &p2));
}

fn sort_desc<T: PartialOrd>(mut list: Vec<T>) -> Vec<T> {
    list.sort_by(|a, b| b.partial_cmp(a).unwrap());
    list
}

fn compare_points<T: PartialOrd + Copy>(p1: &Point<T>, p2: &Point<T>) -> bool {
    p1.x < p2.x && p1.y < p2.y
}

#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_point() {
        let p: Point<i32> = Point { x: 5, y: 10 };
        assert_eq!(*p.x(), 5);
        assert_eq!(*p.y(), 10);
    }

    #[test]
    fn test_generic_pair() {
        let pair: Pair<i32, String> = Pair::new(42, String::from("test"));
        assert_eq!(pair.first, 42);
        assert_eq!(pair.second, "test");
    }

    #[test]
    fn test_generic_container() {
        let c1 = Container::new(42);
        assert_eq!(c1.value, 42);

        let c2 = Container::new("hello");
        assert_eq!(c2.value, "hello");
    }

    #[test]
    fn test_largest() {
        let numbers = vec![1, 5, 3, 9, 2];
        assert_eq!(largest(&numbers), 9);

        let chars = vec!['a', 'z', 'm', 'b'];
        assert_eq!(largest(&chars), 'z');
    }
}
