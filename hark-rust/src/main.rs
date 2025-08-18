fn main() {
    // 1. stack / heap
    // stack_fn();   // Call the function that uses stack memory
    // heap_fn();    // Call the function that uses heap memory
    // update_string();  // Call the function that changes size of variable at runtime

    

}

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
