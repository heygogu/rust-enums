// #[derive(Debug)]
// enum Direction {
//     East,
//     _West,
//     _North,
//     _South,
// }

// fn main() {
//     let dir = Direction::East;
//     //let us print it
//     print_direction(dir);
// }

// fn print_direction(d: Direction) {
//     print!("{:?}", d);
// }

// =================================================

//enums with values

// enum Shape {
//     Circle(f64),
//     Rectangle(f64, f64),
// }

// impl Shape {
//     fn area(&self) {
//         match &self {
//             Shape::Circle(val) => println!("{}", val * val),
//             Shape::Rectangle(w, h) => println!("{}", w * h),
//         }
//     }
// }
// fn main() {
//     //let us find out the area
//     let c = Shape::Circle(2.1);
//     let r = Shape::Rectangle(3.2, 5.7);

//     //find the area of both
//     //1. First way
//     print_area(&c);
//     print_area(&r);

//     //2. Second way by implementing a function on the enums
//     //similar patterns are available on the structs
//     c.area();
//     r.area()
// }

// fn print_area(s: &Shape) {
//     match s {
//         Shape::Circle(val) => println!("{}", val * val),
//         Shape::Rectangle(w, h) => println!("{}", w * h),
//     }
// }

// ======================================================
//Result Enum

// use std::fs;

// fn main() {
//     //let us read a file here
//     // Result enum -> <String,Error> -> writing here as the typing
//     //suggestion won't show on github

//     let f = fs::read_to_string("a.txt");
//     // now I know it may return an error
//     // Below line will make the thread panic
//     // println!("{}", f.as_ref().unwrap());

//     // So to handle that I can use the result enum
//     match f {
//         Ok(content) => println!("{}", content),
//         Err(error) => println!("{}", error),
//     }
// }

// ========================================================
// Defining custom result enum

// use std::fs;

// pub struct FileReadError {
//     message: String,
// }

// fn main() {
//     let contents = read_file("a.txt".to_string());

//     match contents {
//         Ok(content) => println!("{}", content),
//         Err(error) => println!("{}", error.message),
//     }
// }

// fn read_file(s: String) -> Result<String, FileReadError> {
//     //start reading a file
//     let f = fs::read_to_string(s);
//     match f {
//         Ok(content) => Ok(content),
//         Err(_error) => {
//             let err = FileReadError {
//                 message: String::from("There was an error reading the file"),
//             };
//             Err(err)
//         }
//     }
// }

// ========================================================

//Option Enum > Some,None

fn find_char_o(str: String) -> Option<usize> {
    for (index, character) in str.chars().enumerate() {
        if character == 'o' {
            return Some(index);
        }
    }
    return None;
}
fn main() {
    let str = String::from("rhit");
    //finding the index of 'o' if present

    match find_char_o(str) {
        Some(idx) => println!("Found at {}", idx),
        None => println!("Give string does not contains char o"),
    }
}
