// https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum/c_like.html

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    // https://stackoverflow.com/questions/78566874/what-does-02-in-rusts-printlns-formatting-syntax-do

    // :06 here looks if there are 6 characters, if not it will pad it with leading zeros
    println!("roses are #{:06x}", Color::Red as i32);
    println!("plants are #{:06x}", Color::Green as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}