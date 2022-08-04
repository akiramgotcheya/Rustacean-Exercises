
fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}

// comparing a number and a reference to a number is not allowed 
// bcoz they are different types. we must use the dereference operator to follow reference to the value its pointing to.
