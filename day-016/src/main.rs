// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing

fn main() {
    let x = 10;
    let x = x + 1;
    {
        let x = x*2;
        // the below will print 22
        println!("The value of x is: {x}");

    }
    // the following prints 11
    println!("The value of x is: {x}");

    let some_word  = "papa";
    println!("The length of the variable some_word is: {}", some_word.len());
    let some_word = some_word.len();
    println!("Now the variable some_word is a number : {}",some_word);

    let mut spaces = "    ";
    // the below will give an error
    // We’re not allowed to mutate a variable’s type
    // spaces = spaces.len();
}
