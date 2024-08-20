use std::fmt;

#[derive(Debug)]
struct Person<'a> { // what is 'a
    name: &'a str, // what is & and str
    age: u8 // what is u8 - unsigned 8 bit integer
}

/*
Because the struct definition ties it to a referenced object 
(in this case, every struct Person instance is referencing a &str) you want to 
specificly declare an arbitary lifetime and tie these two things together: 
You want a struct Person instance to only live as long as its referenced object 
(hence Person<'a> and name: &'a str) so dangling references after each other's death 
is avoided
*/

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Person<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, " The name is {0} and the age is {1} ", self.name, self.age)
    }
}

// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/lifetimes.html

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
    println!("{}", peter);
}