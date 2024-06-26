// Slice with indices

fn main() {
    let s = String::from("hello");
    
    let word = first_word(&s);

    println!("the first word is: {}", word);
    
    let my_string = String::from("hello world");
    
    let word = first_word(&my_string[0..6]);
    
    let word = first_word(&my_string);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
         if item == b' ' {
             return &s[0..i];
         }
    }
    &s[..]
}
