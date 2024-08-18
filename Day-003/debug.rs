// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
#[allow(dead_code)]
struct UnPrintable(i32);

// If we try this
// println!("{}", UnPrintable(3));
// It will throw an error that ' it doesn't implement `std::fmt::Display'


// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

fn main() {
    // but if you try this
    // println!("{}", DebugPrintable(3))
    // it still won't work as we are using the fmt::Display marker which is {}
    // if we use the following
    println!("With the marker :? {:?}", DebugPrintable(3)); // will print "DebugPrintable(3)" exactly
    
    println!("With the marker :#? {:#?}", DebugPrintable(3));
    // we can also use pretty printing with "{:#?}", with which we get
    /* 
    * DebugPrintable(
    * 3,
    * )
    */
}

