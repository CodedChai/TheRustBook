fn main() {
    let mut s1 = String::from("hello");

    //mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.
    change_via_reference(&mut s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;
}

// You can only alter something via reference if a mutable reference is passed
fn change_via_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // so goes out of scope but since it was referenced nothing happens
