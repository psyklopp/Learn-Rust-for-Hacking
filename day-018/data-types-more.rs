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
    println!("the cat is : {}", heart_eyed_cat);

    // Data types - compound types
    // Tuples

    let tup : (bool, f32, i32) = (true, 4.20, 69);
    println!("The tuple is : {:?}", tup); // cannot use {} to display as Display::Formatter hasn't been implemented yet

    // The below is destructuring, we can also access the elements by tuple.[0-based positioning]
    let (a, b, c) = tup;
    println!("The value of a is {a}, \nThe value of b is {b}, \nThe value of c is {0}", tup.2);

    // Arrays -> let variable_name : [type ; size]
    let my_array: [i32; 5] = [1, 3, 5, 7, 9];
    println!("The array is : {:?} and the last element is {}", my_array, my_array[4]);

    // let variable_name = [element_to_be_duplicated ; size]
    let my_other_array = [3; 5];
    println!("The other array is : {:?}", my_other_array);
}