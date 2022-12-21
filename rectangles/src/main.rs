#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area_method(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height:size
        } //this function doesnt reference self and instead can be used to make a new square rectangle
    } // these kind of functions are usually used for constructors
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.",
        area_with_tuples(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.",
        area_with_struct(&rect2)
    );

    println!("Rectangle 2 is: {:#?}", &rect2);
    
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    dbg!(&rect3);

    println!("Fuck yeah!");
    println!("The area of rectangle is {}", rect2.area_method());
    println!("The area of rectangle is {}", rect3.area_method());

    if rect2.width() {
        println!("Rectangle has a non zero width of {}", rect2.width);
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

