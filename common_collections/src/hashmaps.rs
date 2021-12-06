use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name); 

// updating values
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

for (key, value) in &scores {
    println!("{}: {}", key, value); 
}

//Updating a value based on the old value

let text = "hello world wonderful world";

let mut map = HashMap:new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map); // prints {"world": 2, "hello": 1, "wonderful": 1}

// only inserting a value if the key has no value
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap <_, _> = // rust will infer the types used in the hashmap
    teams.into_iter().zip(initial_scores.into_iter()).collect();

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value); // data that is moved and not copied will transfer ownership to the hashmap
// field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    //
