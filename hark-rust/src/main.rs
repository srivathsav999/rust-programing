// fn main() {
    // 1. stack / heap
    // stack_fn();   // Call the function that uses stack memory
    // heap_fn();    // Call the function that uses heap memory
    // update_string();  // Call the function that changes size of variable at runtime

    

// }

// stack / heap
// fn stack_fn() {
//     // Declare a few integers on the stack
//     let a = 10;
//     let b = 20;
//     let c = a + b;
//     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// }

// fn heap_fn() {
//     // Create a string, which is allocated on the heap
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let combined = format!("{} {}", s1, s2);
//     println!("Heap function: Combined string is '{}'", combined);
// }

// fn update_string() {
//     // Start with a base string on the heap
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);
//     println!("length: {}, capacity: {}, pointer: {:p}", s.len(), s.capacity(), s.as_ptr());

//     // Append some text to the string
//     s.push_str(" and some additional text");
//     println!("After update: {}", s);
//     println!("length: {}, capacity: {}, pointer: {:p}", s.len(), s.capacity(), s.as_ptr());
// }


// structs
// struct User {
//     name: String,
//     age: u32,
//     email: String,
// }


// fn main() {
//     let user1 = User {
//         name: String::from("John"),
//         age: 30,
//         email: String::from("john@example.com"),
//     };

//     println!("{} is {} years old and can be contacted at {}", user1.name, user1.age, user1.email);
// }


//hashmaps 


// use std::collections::HashMap;

// fn main(){
//     let mut users = HashMap::new();

//     users.insert(String::from("hark"), 1);
//     users.insert(String::from("hark-1"), 2);
//     users.insert(String::from("hark-2"), 3);
//     users.insert(String::from("hark-3"), 4);

//     let first_user = users.get("hark");

//     match first_user {
//         Some(soem) => println!("user is {}", soem),
//         None => println!("nothing found"),
//     }
// }

//tuples

use std::collections::HashMap;


fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32>{

    let mut hm = HashMap::new();
    for (key, value) in vec{
        hm.insert( key, value);
    }
    return hm;
}
fn main(){

    let input_vec = vec![(String::from("hey"), 1),(String::from("hello"), 2)];
    let hm = group_values_by_keys(input_vec);

    println!("here is the hm {:?}", hm)
}
