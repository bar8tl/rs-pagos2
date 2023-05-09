/*******************************************************************************
** rblib.rs: Library functions for Rust applications [20220406-BAR8TL]         *
*******************************************************************************/
#![allow(unused)]

// Perform rounding of floating point numbers to specific decimal positions
pub fn rb_round(x: f32, y: u32) -> f32 {
  let y = 10i32.pow(y) as f32;
  (x * y).round() / y
}

// Determines if a number is into one numbers list
pub fn contains(s: &Vec<usize>, e: &usize) -> bool {
  for a in s {
    if a == e {
      return true;
    }
  }
  return false;
}

// Indicates if a char string matches one pattern
pub fn pass_filter(ifilt: &String, filen: &str) -> bool {
  true
}

// Display the data type of one object
pub fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}
