fn main() 
{   //s is not valid here, it's not yet declared
    let s = "hello"; // s is valid from this point forward (literal string stored on stack)

    doee();
    // do stuff with s
} // this scope is now over, and s is no longer valid


fn String() {

    let mut s = String::from("hello"); //allocated on the heap; valid from this point forward

    s.push_str(", world!"); //push_str() appends a literal to a String

    println!("{}", s);
} // scope is over and s is no longer valid

fn ex1() { //data stored on the stack at runtime are always going to have deep copies; data stored on the stack have the copy trait

    let x = 5;

    let y = x; // data is copied over since the data is stored on the stack

    println!("{} {}", x, y);
}

fn ex2() {
    let s1 = String::from("hello");
    let s2 = s1; // since s1 data is stored on the heap, s2 only copies the pointer to s1's data (known as a move)

    //println!("{} {}", s1); 
    // when moving, first data is rendered invalid 
}

fn ex3() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // instead of moving, .clone() makes a deep copy like with data on copied on the stack

    println!("s1 = {}, s2 = {}", s1, s2);
}




fn doee() {

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function
                        // ... and so is no longer valid here
    //println!("{}", s);

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still use x afterward

    println!("{}", x);

    let s3 = gives_ownership(); // gives_ownership moves its return value into s1

    let s4 = String::from("hello"); // s4 comes into scope

    let s5 = takes_and_gives_back(s4);
    // s4 is moved into
    // takes_and_gives_back, whcih also moves
    // its return value into s5

    let (s6, len) = calculate_length(s1);
} //Here, s5 goes out of scope and is dropped. s4 goes out of scope but was
// moved, so nothign happens. s1 goes out of scope and is dropped

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {

    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
