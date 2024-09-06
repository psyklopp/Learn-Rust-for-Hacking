fn main() {

    /* Variables and Immutability
     */
    let x = 5;
    println!("Value of x: {}", x);
    
    // the following throws error, cannot assign twice to immutable variable
    // x = 6;
    // println!("Value of x: {}", x);

    println!("Now using mutable y as in let mut y = 10");

    let mut y = 10;
    println!("Value of y: {}", y);
    
    // the following throws error, cannot assign twice to immutable variable
    y = 11;
    println!("Value of y: {}", y);

    /*
    Now talking about constants
     */

    // Constants aren’t just immutable by default—they’re always immutable
    // Uppercase with underscores between words

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Value of const THREE_HOURS_IN_SECONDS is : {}", THREE_HOURS_IN_SECONDS);
}
