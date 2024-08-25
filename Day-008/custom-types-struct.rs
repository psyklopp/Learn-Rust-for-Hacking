/*
Rust custom data types are formed mainly through the two keywords:

struct: define a structure
enum: define an enumeration

*/

#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// Tuple struct
struct Pair(i32, f32);

// Struct with fields
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn display_point(point:Point) -> () {
    println!("The coordinates are: \n x: {}, y: {}", point.x, point.y);
}

fn main() {
    let name = String::from("Addy");
    let age = 21;
    let addy = Person {name, age};

    println!("{:?}", addy);

    let first_point = Point{x: 4.5, y: 2.5};
    let second_point = Point{x: 7.5, y: 1.5};
    // we can also use the concept of a straight line on x-axis, consisting of 2 points x and y on a 2D graph
    // let second_point = Point{x: 7.5, ..first_point}; --> the point only moves along the x-axis
    display_point(first_point);
    display_point(second_point);
    
    // Below from Rust by Example   

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..another_point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;
    println!("left-edge : {}, top_edge : {}", left_edge, top_edge);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}