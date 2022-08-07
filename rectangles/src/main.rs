#![allow(unused_variables, dead_code)]
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    tuple_rectangle();
    struct_rectangle();

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// we can refactor with tuples
fn tuple_rectangle() {
    let rect1 =  (50, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

}
 fn area_tuple(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
 }


 // refactoring with structs

 #[derive(Debug)]
 struct Rectangle {
    width: u32,
    height: u32,
 }

 fn struct_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    //using a function
    println!(
        "The are of the rectangle is {:?} square pixels.",
        area_struct(&rect1)
    );

    // using the method
    println!(
        "The are of the rectangle is {:?} square pixels.",
        rect1.area()
    );


    // using method with additional parameters
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let rect4 = Rectangle::square(4);
    dbg!(rect4);
 }

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// converting the area_struct function to be a method of a Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    //methods like this are getters, often named the same as their field. This allows for a read only through the use of pub method and private field
    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // below is an associated function, since the function does not take any form of self as a parameter
    // returns a rectangle based on one given dimension
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
// rust knows whether the method has the parameter as &mut, &self, or self and uses the matching struct for the call (&p).method(&p)