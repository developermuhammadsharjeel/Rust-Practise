// When you have many choices, using match is easier than writing lots of if...else.

// match is used to select one of many code blocks to be executed:

//  Rust Mathes concept is quite similar to Switch statements in other languages like c++

// Example...................

fn main() {
  let day = 4;

  match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
  }


//........................Multiple Matches
  let day = 6;

  match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
  }
}




  

