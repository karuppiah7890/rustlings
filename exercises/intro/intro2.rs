// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.

fn main() {
    println!("Hello {}!", "World");
    // println!("Hello {}!", "World", 2); // this is wrong!
    println!("Hello {1}, {0}", 1, 2);
    // println!("Hello {1}, {100}", 1, 2) // this is wrong
    // println!("Hello {1}, {100}, {0}", 1, 2, 3) // this is wrong

    println!("Hello {1}, {0}, {1}", 3, 4);

    println!("Hello {1}, {1}, {}", 5, 6);

    println!("Hello {}, {0}, {}", 7, 8);

    println!("Hello {}, {0}, {0}", 9);

    // println!("Hello {}, {0}, {0}", 10, 11) this is wrong
}
