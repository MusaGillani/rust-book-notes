#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// an impl block has implementations of methods associated to a struct
// notice the name of the impl block is the same as the struct
impl Rectangle {
    // self is the object instance this method is being called on,
    // self is always the first parameter.
    // it can be borrowed (like in this case), a mutable reference or in some cases owned (moved).
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// structs allow us to have multiple implementation blocks
// whatever is written in this can be written in the impl block above
impl Rectangle {
    // associated functions
    // does not take in the self arg
    // as it is not related to the object instance.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}
fn main() {
    let rect: Rectangle = Rectangle {
        height: 50,
        width: 30,
    };

    let rect1: Rectangle = Rectangle {
        height: 40,
        width: 20,
    };

    let rect2: Rectangle = Rectangle {
        height: 60,
        width: 40,
    };

    println!("rect: {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect)
        rect.area()
    );

    // using associated functions
    let rect3 = Rectangle::square(20);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("square is: {:#?}", rect3);
}

fn _area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
