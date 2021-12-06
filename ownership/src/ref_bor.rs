fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}', is {}.", s1, len);

    //change(&s1); wont work, references are immutable by default
    change(&mut s1); // you can only have one mutable reference to a particular piece of data in a particular scope
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // this code will fail
    // helps prevent data races at compile time
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does nothave ownership of what
  // it refers to, nothing happens
 
fn change(some_string: &mut String) {
    some_string.push_str(", world"); // references are immutable by default

}

fn ex1() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems

    let r2 = &mut s;
}

fn ex2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    // cannot have mutable and immutable references in the same scope
    // immutable references expect the value to stay the same

    println!("{}, {}, and {}", r1, r2, r3);

}

fn ex3() {
    let mut s = String::from("hello");

    let r1 = &s; //no problem
    let r2 = &s; // no problem
    println("{} and {}", r1, r2); 
    //variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn main2() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. It's memory goes away.
   // s is deallocated but the function tries to return a reference to it
fn dangle_solution() -> String {
    let s = String::from("hello");
    s
}
//tl;dr
// at any given time, you can have either one mutable reference or any number of immutable
// references
// references must always be valid
