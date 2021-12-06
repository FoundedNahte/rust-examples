fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_00;

    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64

    let y: f32 = 3.0; //f32

    let f: bool = false;

    let c = 'z'; //unicode scalar value

    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple is a general way to group (contents do not have to be the same type)

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let a = [1, 2, 3, 4, 5]; //array (contents must be same type)

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // type i32 and length 5

    let a = [3; 5]; // [3, 3, 3, 3, 3]



    let y = {
        let x = 3;
        x + 1
    };

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() { // 4 3 2 1 
        println!("{}!", number);
    }
}

fn another_function(x: i32) -> i32 {
   return x; 
}
