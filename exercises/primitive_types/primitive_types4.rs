// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap

// https://doc.rust-lang.org/book/ch04-03-slices.html

// https://doc.rust-lang.org/nomicon/coercions.html

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..=3];

    assert_eq!([2, 3, 4], nice_slice)
}
