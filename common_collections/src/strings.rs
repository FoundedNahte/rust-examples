 let mut s = String::new();

let data = "initial contents";

let s = data.to_string();

let s = "initial contents".to_string();

let s = String::from("initial contents");

// Strings are utf-8 encoded


let mut s = String::from("foo");
s.push_str("bar"); //push_str to append a string slice
// s = "foobar"

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);

let mut s = String::from("lo");
s.push('l'); // add a char

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

// for more complicated string combining, we can use the format! macro
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);

let s1 = String::from("hello");
let h = s1[0]; // errors; Rust strings don't support indexing

//methods for iterating over strings

for c in "abcd".chars() {
    println!("{}", c);
}

for b in "abcd".bytes() {
    pritnln!("{}", b);
}
