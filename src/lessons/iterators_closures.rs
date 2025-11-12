//! # Iterators & Closures
//!
//! 目标：理解 Rust 中的迭代器和闭包
//!
//! ## 要点
//! - 迭代器：惰性地遍历集合中的元素
//! - 迭代器 trait：`next()` 方法返回 `Option<Self::Item>`
//! - 闭包：匿名函数，可以捕获环境变量
//! - 闭包有三种类型：`Fn`、`FnMut`、`FnOnce`
//! - 迭代器适配器：`map`、`filter`、`fold` 等
//! - 消费适配器：`collect`、`sum`、`for_each` 等
//!
//! ## 常见坑
//! - 迭代器是惰性的，需要消费适配器才能执行
//! - 闭包捕获所有权可能导致后续无法使用变量
//! - 在迭代过程中修改集合可能导致问题
//!
//! ## 运行
//! `cargo run -- 16_iterators_closures`

pub fn run() {
    println!("=== 迭代器基础 ===");
    demo_iterator_basics();

    println!("\n=== 迭代器适配器 ===");
    demo_iterator_adapters();

    println!("\n=== 闭包基础 ===");
    demo_closures();

    println!("\n=== 闭包捕获 ===");
    demo_closure_capture();
}

fn demo_iterator_basics() {
    let v = vec![1, 2, 3];

    let mut iter = v.iter();
    println!("First: {:?}", iter.next());
    println!("Second: {:?}", iter.next());
    println!("Third: {:?}", iter.next());
    println!("Fourth: {:?}", iter.next());

    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);

    let collected: Vec<_> = v.iter().collect();
    println!("Collected: {:?}", collected);

    for val in v.iter() {
        println!("Value: {}", val);
    }
}

fn demo_iterator_adapters() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Fold sum: {}", sum);

    let product: i32 = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Fold product: {}", product);

    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);

    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);

    let any_negative = numbers.iter().any(|x| *x < 0);
    println!("Any negative: {}", any_negative);

    let chained: Vec<_> = numbers
        .iter()
        .filter(|x| **x > 2)
        .map(|x| x * 3)
        .collect();
    println!("Filter > 2 then * 3: {:?}", chained);
}

fn demo_closures() {
    let add = |x, y| x + y;
    println!("Add: 5 + 3 = {}", add(5, 3));

    let square = |x| x * x;
    println!("Square: 5^2 = {}", square(5));

    let make_greeting = |name| format!("Hello, {}!", name);
    println!("{}", make_greeting("Rust"));

    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("Count: {}", increment());
    println!("Count: {}", increment());
    println!("Count: {}", increment());

    let nums = vec![1, 2, 3, 4, 5];
    let squares: Vec<_> = nums.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    let even_squares: Vec<_> = nums
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Even squares: {:?}", even_squares);
}

fn demo_closure_capture() {
    let x = 10;
    let print_x = || println!("x = {}", x);
    print_x();

    let mut count = 0;
    let items = vec![1, 2, 3, 4, 5];
    items.iter().for_each(|_| {
        count += 1;
    });
    println!("Count after foreach: {}", count);

    let mut nums = vec![1, 2, 3, 4, 5];
    let multiplier = 2;
    nums.iter_mut().for_each(|n| *n *= multiplier);
    println!("Multiplied nums: {:?}", nums);

    move_closure();
}

fn move_closure() {
    let s = String::from("hello");
    let take_s = move || println!("Moved: {}", s);
    take_s();
    // s is moved, cannot use here
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn demo_custom_iterator() {
    let mut counter = Counter::new();
    println!("Custom iterator:");
    while let Some(num) = counter.next() {
        println!("  {}", num);
    }

    // Using iterator methods
    let sum: u32 = Counter::new().sum();
    println!("Sum of counter: {}", sum);

    let powers: Vec<u32> = Counter::new().map(|x| x * x).collect();
    println!("Counter squares: {:?}", powers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_sum() {
        let v = vec![1, 2, 3, 4, 5];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_map_collect() {
        let v = vec![1, 2, 3];
        let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_filter() {
        let v = vec![1, 2, 3, 4, 5];
        let evens: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();
        assert_eq!(evens, vec![2, 4]);
    }

    #[test]
    fn test_fold() {
        let v = vec![1, 2, 3, 4, 5];
        let sum = v.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_closure() {
        let add = |x, y| x + y;
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_closure_capture() {
        let x = 10;
        let add_x = |y| x + y;
        assert_eq!(add_x(5), 15);
    }

    #[test]
    fn test_custom_iterator() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_iterator_methods() {
        let sum: u32 = Counter::new().sum();
        assert_eq!(sum, 15); // 1+2+3+4+5
    }
}
