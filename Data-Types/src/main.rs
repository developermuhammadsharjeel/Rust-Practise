//........... Rust data Types
let num = 5;         // integer
let double-value = 5.99;   // float
let letter = 'D';    // character
let bool = true;     // boolean
let message = "Hello";  // string

/*
1-->In Rust we do not need to be declared variables  with a specified type (like "String" for text or "Int" for numbers )
2-->The type of a variable is decided by the value you give.
For example if you declared let num = 17 rust automaticaly consider it Integer value
and if you declare let num = "twenty two" its considered as string
so we dont need to declare data types in rust
*/


// //......................................................
// it is possible to explicitly tell Rust what type a value should be:

let my_num: i32 = 5;          // integer
let my_double: f64 = 5.99;    // float
let my_letter: char = 'D';    // character
let my_bool: bool = true;     // boolean
let my_text: &str = "Hello";  // string

//there is also 2 types for integers
//1- i32 signned integer positive or negative
//2- u32 only for positive integers
