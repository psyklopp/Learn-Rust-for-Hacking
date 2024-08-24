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
    println!("The coordinates are: \nx: {}, y: {}", point.x, point.y);
}

fn main() {
    let name = String::from("Addy");
    let age = 21;
    let addy = Person {name, age};

    println!("{:?}", addy);

    let first_point = Point{x: 4.5, y: 2.5};
    //let second_point = Point{x: 7.5, y: 1.5};
    // we can use the concept of a line consisting of 2 points x and y on a 2D graph
    let second_point = Point{x: 7.5, ..first_point};
    display_point(first_point);
    display_point(second_point);
}