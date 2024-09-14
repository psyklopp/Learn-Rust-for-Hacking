fn print_intro(intro_str: &str) {
    println!("I am in other function: {intro_str}");
}

fn main() {
    let hello_string = "Hello world"; // a statement
    println!("I am in the main function. And here is the introduction string: ");
    print_intro(hello_string);

    // let x = (let y = 6); 
    // gives error. A statement doesn't return a value

    let y = {
        let x = 3;
        x + 1 // doesn't have ending semicolon
    }; // this is an expression. 

    println!("Value of y = {y}"); // gives 4

    let ans = add_one(y);
    println!("The value of ans is : {ans}");
}

fn add_one(y: i32) -> i32 {
    y + 1 // ; -> adding a semicolon produces an error as it becomes a statement then
}