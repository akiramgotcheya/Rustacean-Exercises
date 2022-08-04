
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world");

  println!("{}", &s);
}

// Ln6 borrowed mutable s
// Ln14 println to ouutput hello world
