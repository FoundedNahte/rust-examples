fn main() {
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s);

    s.clear();

    let my_string = String::from("hello world");

    // 'first_word' works on slices of 'String's, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 'first_word' also works on references to 'String's, which is equivalent to a slice of the
    // whole 'String'
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // 'first_word' works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_wrod(&my_string_literal[..]);

    // Because string literals *are* string slices already, this works too, without the slice
    // syntax!
    let word = first_word(my_string_litearl);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}


fn first_word(s: &str) -> &str {// string slice written as &str; taking parameter of &str allows the use of string literals and String
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
