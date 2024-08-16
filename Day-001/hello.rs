//Macros are created using the macro_rules! macro

macro_rules! say_hello {
    () => {
        println!("This is psyklopp and I used macro I made myself, to print this second line.")
    }
}

// The main function
fn main() {
    println!("Hello World!");
    /* println! is a macro.
     * let's create a macro on our own - https://doc.rust-lang.org/rust-by-example/macros.html */
    say_hello!()
}

// I am using Windows system currently and it created two files from hello.rs
// One - the binary file - hello.exe
// Two - hello.pdb - seems to contain extra information for the Windows system for debugging. How? I don't care. For now.