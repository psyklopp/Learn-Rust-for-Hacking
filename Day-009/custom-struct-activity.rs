// Struct with fields
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

/* 
 Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
*/

fn rect_area(rect:Rectangle) -> (){

    // destructure the rectangle to get the points
    let Rectangle {
        top_left: point_one, 
        bottom_right: point_second
    } = rect; 

    // destructure the points
    let Point {x: x1, y: y1} = point_one;
    let Point {x: x2, y: y2} = point_second;

    println!("x1: {}, y1: {}, x2: {}, y2: {}", x1, y1, x2, y2);

    // calculate the area as (x1-x2)*(y1-y2)
    println!("The area of rectangle is: {}", ((x1-x2)*(y1-y2)).abs());
}

/* 
Add a function square which takes a Point and a f32 as arguments, and 
returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
*/

fn main() {
    let first_point = Point{x: 4.5, y: 2.5};
    let second_point = Point{x: 7.5, y: 1.5};
    
    let my_rectangle = Rectangle {
        top_left: first_point,
        bottom_right: second_point
    };

    rect_area(my_rectangle);
    println!("End");
}