//........ Basic Variables types in Rust

/*

let → immutable
let mut → mutable
const for values that never change


*/

fn main() {
    //variable 1 == let
    let name ="dveloper sharjeel";
    println!("{}",name);
    // Rust uses {} as a placeholder in println!() to show variable values.

    
    // we can not intialize name variable again without making it mutable == let mut
    // name ="Rashid";
    // println!("{}",name); //Error . we can assign let variable again


    // variable 2 ==let mut .....................
    let mut friend="Ahmad Raza";
    println!("{}",friend);
   // Now we can reassign value of variable because its muttable now
    friend="Moazzam";
    println!("After chnaging variable friend the new name is {}",friend,);

    //variable 3 == const .........................
    const PI: f64 = 3.14159;
     //values can not be changed because its constant value
    
     println!("PI = {}", PI);
    
   
}
