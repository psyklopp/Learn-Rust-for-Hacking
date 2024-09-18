fn main() {
    let mut x = 1;
    let maxlimit = 5;
    let result = loop {
        if x > maxlimit {
            break x;
        }
        println!("Hello there!");
        x = x + 1;
    };

    println!("The counter reached is: {:?}", result);
    println!("Star Pattern - Pyramid");

    // Star pattern
    //   *  
    //  ***      
    // *****
    //*******
    // let row :i32 = 4;
    // let mut x = 0;
    // loop {
        
    //     let mut y = 1;
    //     loop {
    //         if y < row - x {
    //             print!{" "};
    //             y = y + 1;
    //         } else {
    //             break;
    //         }

    //     }

    //     let mut z = 0;
    //     loop {
    //         if z < 2*x + 1 {
    //             print!("*");
    //             z = z + 1;
    //         } else {
    //             break
    //         }
    //     }

    //     x = x + 1;
    //     println!();
    //     if x >= row {
    //         break;
    //     }

    // }

    let row :i32 = 4;
    let mut x = 0;

    while x < row  {
        let mut y = 1;
        while y < row - x {
            print!(" ");
            y = y + 1;
        }
        
        let mut z = 0;
        while z < 2*x + 1 {
            print!("*");
            z = z + 1;
        }

        println!();
        x = x + 1;
    }

    println!("- end -");

}