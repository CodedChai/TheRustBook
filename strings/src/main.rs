fn main() {
    let hello = String::from("こんにちは");

    println!("{}", hello);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used because the operator is implemented like -> fn add(self, s: &str) -> String {
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s4 is {}", s4);

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // String slices go by bytes, not necessarily characters/graphemes
    println!("s slice {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
