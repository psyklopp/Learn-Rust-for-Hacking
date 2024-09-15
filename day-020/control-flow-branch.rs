/* https://doc.rust-lang.org/book/ch03-05-control-flow.html
 */

fn main() {
    let number = 32;
        println!("Number is {number}");
    if number % 2 == 0 {
        println!("Divisible by 2.");
    } else {
        println!("Not divisible by 2.");
    }

    let luck_condition = false;
    let final_number = if luck_condition { 42 } else { 13 };

    println!("This is the end. Your luck is {} and the final number is {}", luck_condition, final_number);
}