// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    SOCAnalyst,
    PenetrationTester,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Stage::{Beginner, Advanced};
    // Automatically `use` each name inside `Role`.
    use crate::Role::*;

    // Equivalent to `Stage::Beginner`.
    let stage = Beginner;
    // Equivalent to `Role::Penetration_Tester`.
    let role = PenetrationTester;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        SOCAnalyst => println!("They are in the shadows. Mostly."),
        PenetrationTester => println!("They are in the shadows. Like Ninjas."),
    }
}