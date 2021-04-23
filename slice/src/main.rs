fn main() {
    let words = String::from("hello world");

    let word = first_word_index(&words); // word will get the value 5

    println!("{}", word);

    // words.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // String slices get us direct references to parts of the string
    let hello = first_word_slice(&words);
    let world = &words[6..];

    println!("{} {}", hello, world);

    let a = [1, 2, 3, 4, 5];

    let _slice = &a[1..3];
    // slices also work for arrays
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // first element is index, second is reference to value neat!
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
