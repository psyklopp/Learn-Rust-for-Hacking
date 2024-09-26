fn main() {
    let mut count = 10;
    // loop lable below - have to use a ' before the start of the label name
    'counting_up: loop {
        let count_final = 4;
        if count == count_final {
            break 'counting_up; // use label to break a specific loop
        }
        println!("{count}..");
        count -= 1;
    }

    // for loop is good for iterating collections

    for number in (1..5).rev() {
        println!("{number}..");
    }

    println!("Liftoff!")
}