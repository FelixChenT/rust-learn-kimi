//! # Methods & Associated Functions
//!
//! 目标：理解 Rust 中的方法（method）和关联函数（associated function）
//!
//! ## 要点
//! - 方法是定义在结构体、枚举或 trait 对象上的函数，第一个参数是 `self`
//! - 关联函数（不使用 self）类似于其他语言的静态方法
//! - `self`、`&self`、`&mut self` 表示不同的所有权关系
//! - 方法可以访问结构体的私有字段
//! - 使用 `impl` 块定义方法和关联函数
//!
//! ## 常见坑
//! - 忘记方法会自动借用 self（&self）
//! - 混淆方法和关联函数的区别
//! - 在多个 impl 块中定义方法时需要小心
//!
//! ## 运行
//! `cargo run -- 11_methods_assoc_fn`

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Rectangle {
    // 方法：通过 &self 借用实例
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数：没有 self 参数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

pub fn run() {
    println!("=== 方法调用 ===");
    demo_methods();

    println!("\n=== 关联函数 ===");
    demo_associated_functions();

    println!("\n=== 方法链式调用 ===");
    demo_method_chaining();

    println!("\n=== 多个 impl 块 ===");
    demo_multiple_impl();
}

fn demo_methods() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle: {:?}", rect);
    println!("Area: {} square pixels", rect.area());

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    let circle = Circle { radius: 5.0 };
    println!("Circle: {:?}", circle);
    println!("Area: {:.2}", circle.area());
    println!("Circumference: {:.2}", circle.circumference());

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("Distance from {:?} to {:?}: {}", p1, p2, p1.distance(&p2));
}

fn demo_associated_functions() {
    let square = Rectangle::square(20);
    println!("Square: {:?}", square);
    println!("Square area: {}", square.area());

    let circle = Circle::new(10.0);
    println!("Circle: {:?}", circle);
    println!("Circle area: {:.2}", circle.area());

    let origin = Point::origin();
    println!("Origin: {:?}", origin);
}

fn demo_method_chaining() {
    let area = Rectangle::square(10).area();
    println!("Square of 10 area: {}", area);
}

fn demo_multiple_impl() {
    impl Rectangle {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }

    let sq = Rectangle::square(15);
    println!("Is square? {}", sq.is_square());

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Is square? {}", rect.is_square());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_can_hold() {
        let rect1 = Rectangle {
            width: 30,
            height: 30,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 10,
        };
        assert!(rect1.can_hold(&rect2));
        assert!(!rect2.can_hold(&rect1));
    }

    #[test]
    fn test_square_associated_fn() {
        let square = Rectangle::square(10);
        assert_eq!(square.width, 10);
        assert_eq!(square.height, 10);
        assert!(square.is_square());
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 1.0 };
        let area = circle.area();
        assert!((area - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_circle_new() {
        let circle = Circle::new(5.0);
        assert_eq!(circle.radius, 5.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_point_origin() {
        let origin = Point::origin();
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
    }
}
