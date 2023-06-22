// fn main() {
//     let length1 = 50;
//     let width1 = 30;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(length1, width1)
//     );
// }

// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

// fn main() {
//     let rect1 = (50, 30);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// fn main() {
//     let rect1 = Rectangle { length: 50, width: 30 };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.length * rectangle.width
// }

// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// fn main() {
//     let rect = Rectangle {
//             length: 50, 
//             width: 30 
//         };

//     println!(
//         "rect is {:#?}.",
//         rect
//     );
// }

// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
//     fn area (&self) -> u32 {
//         self.length * self.width
//     }
// }

// fn main() {
//     let rect = Rectangle {
//         length: 50,
//         width: 30
//     };
//     println!(
//         "the area of the rectangle is {} square pixels.",
//         rect.area()
//     );
// }

// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.length * self.width
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.length > other.length && self.width > other.width
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         length: 50,
//         width: 30
//     };
//     let rect2 = Rectangle {
//         length: 40,
//         width: 10
//     };
//     let rect3 = Rectangle {
//         length: 45,
//         width: 60
//     };

//     println!(
//         "Can rect1 hold rect2? {}", 
//         rect1.can_hold(&rect2)
//     );
//     println!(
//         "Can rect1 hold rect2? {}", 
//         rect1.can_hold(&rect3)
//     );
// }


// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 { // metodo
//         self.length * self.width
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool { // metodo
//         self.length > other.length && self.width > other.width
//     }

//     fn square(size: u32) -> Rectangle { // funcao associada... ex: String::from
//         Rectangle {     // para chamar a funcao -> Rectangle::square(3);
//             length: size,
//             width: size
//         }
//     }
// }


#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // metodo
        self.length * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool { // metodo
        self.length > other.length && self.width > other.width
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle { // funcao associada... ex: String::from
        Rectangle {                     // para chamar a funcao -> Rectangle::square(3);
            length: size,
            width: size
        }
    }
}








