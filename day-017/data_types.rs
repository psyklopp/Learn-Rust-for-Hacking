fn main() {
    // Scalar types - Integer, Floating point, Boolean, Character
    
    // integer 
    // explicitly annotated the data type
    let guess : u32 = 42;
    println!("the value of guess is {guess}");
    
    // floating
    let x = 2.0; // f64 - floating point data type is 64-bit by default
    let y: f32 = 3.0; // f32
    
    // boolean
    let t = true;
    let f : bool = false;

    // character
    let heart_eyed_cat = '😻';
    println!("the cat is : {}", heart_eyed_cat)
}
