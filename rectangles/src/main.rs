#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // not using self: "associated function" (equivalent of a static fn)
    fn square(side: u32) -> Rectangle {
        Rectangle {
            height: side,
            width: side,
        }
    }
}

// Multiple impl blocks for the same struct
impl Rectangle {
    fn can_hold(&self, r: &Rectangle) -> bool {
        self.width >= r.width && self.height >= r.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area(&rectangle)
    );

    println!(
        "The area of \n{:#?}\nis {} square pixels.",
        rectangle,
        rectangle.area(),
    );

    // Note: Rust automatically dereferences pointers when calling methods
    // The following two syntaxes are the same
    // p1.distance(&p2);
    // (&p1).distance(&p2);
    // Rust automatically adds in &, &mut, or * so p1 matches
    // the signature of the method.

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("{:#?}", Rectangle::square(23));
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
