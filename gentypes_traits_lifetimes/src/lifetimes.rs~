//below is an example of a dangling reference, what lifetimes aim to prevent
    
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

// below is a valid version
{
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}

//generic lifetimes in functions

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str { // wont function because the compiler doesnt know whether the returned reference is going to be x or y
    if x.len() > y.len() {
        x
    } else{
        y
    }
}

&i32 // a reference
&'a i32 // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

//lifetime annotations in context of longest function
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
