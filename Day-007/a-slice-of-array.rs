/* Arrays and Slices
 * https://doc.rust-lang.org/rust-by-example/primitives/array.html
 */

use std::mem;

// This function borrows a slice.
fn analyze_slice(slic: &[f64]) {
    println!("First element of the slice: {}", slic[0]);
    println!("The slice has {} elements", slic.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous).
    //let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let xs: [f64; 5] = [1.0, 2.04, 3.02, 4.5, 5.006];

    // All elements can be initialized to the same value.
    let ys: [f64; 500] = [420.69; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    // analyze_slice(&ys[0 .. 500]); --> you get the whole array
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[2 .. 500]); // we have 498 elements here
    

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1{ // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    println!("Now this is where it ends but wait");

    // Out of bound indexing on array causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    println!("{}", xs[..][4]); // causes runtime error
}