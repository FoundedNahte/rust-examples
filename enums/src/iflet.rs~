let some_u8_value = Some(0u8);

if let Some(3) = some_u8_value {
    println!("three");
}
//above same as below
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state);
    _ => count += 1;
}
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
