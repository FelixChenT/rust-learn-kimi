//! # Control Flow: if / loop / while / match
//!
//! 目标：掌握 Rust 中的控制流结构
//!
//! ## 要点
//! - `if` 是表达式，可以有返回值
//! - `loop` 无限循环，可用 `break` 退出并返回值
//! - `while` 条件循环
//! - `for` 遍历集合
//! - `match` 模式匹配，必须穷尽所有可能
//! - `if let` 简化 match 处理单一模式
//!
//! ## 常见坑
//! - `if` 条件必须是 `bool` 类型，不能隐式转换
//! - `match` 必须处理所有情况或添加通配符 `_`
//! - 忘记 `break` 导致无限循环
//!
//! ## 运行
//! `cargo run -- 05_control_flow`

pub fn run() {
    println!("=== if 表达式 ===");
    demo_if();

    println!("\n=== loop 循环 ===");
    demo_loop();

    println!("\n=== while 循环 ===");
    demo_while();

    println!("\n=== for 循环 ===");
    demo_for();

    println!("\n=== match 模式匹配 ===");
    demo_match();

    println!("\n=== if let ===");
    demo_if_let();
}

fn demo_if() {
    let number = 42;

    if number < 0 {
        println!("{} is negative", number);
    } else if number > 0 {
        println!("{} is positive", number);
    } else {
        println!("{} is zero", number);
    }

    // if 作为表达式
    let result = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    println!("{} is {}", number, result);
}

fn demo_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter >= 3 {
            break counter * 2; // loop 可以返回值
        }
    };
    println!("Loop result: {}", result);
}

fn demo_while() {
    let mut number = 5;

    while number > 0 {
        println!("While countdown: {}", number);
        number -= 1;
    }
    println!("Liftoff!");
}

fn demo_for() {
    let arr = [10, 20, 30, 40, 50];

    println!("For loop with array:");
    for element in arr.iter() {
        println!("Value: {}", element);
    }

    println!("For loop with range:");
    for i in (1..=3).rev() {
        println!("Reverse: {}", i);
    }
}

fn demo_match() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 | 5 => println!("Four or Five"),
        6..=10 => println!("Six through Ten"),
        _ => println!("Something else"),
    }

    let opt = Some(5);
    match opt {
        Some(x) => println!("Got a value: {}", x),
        None => println!("Got nothing"),
    }
}

fn demo_if_let() {
    let some_value = Some(42u32);

    // 使用 if let 简化 match
    if let Some(x) = some_value {
        println!("if let matched: {}", x);
    } else {
        println!("if let: no value");
    }

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_expression() {
        let result = if true { 5 } else { 10 };
        assert_eq!(result, 5);

        let result = if false { "yes" } else { "no" };
        assert_eq!(result, "no");
    }

    #[test]
    fn test_loop_return() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter >= 5 {
                break counter * 2;
            }
        };
        assert_eq!(result, 10);
    }

    #[test]
    fn test_while_loop() {
        let mut count = 0;
        let mut i = 5;
        while i > 0 {
            count += 1;
            i -= 1;
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn test_match_all_cases() {
        let x = Some(10);
        let result = match x {
            Some(v) => v,
            None => 0,
        };
        assert_eq!(result, 10);
    }

    #[test]
    fn test_if_let() {
        let x = Some(5);
        let mut result = 0;
        if let Some(v) = x {
            result = v;
        }
        assert_eq!(result, 5);
    }
}
