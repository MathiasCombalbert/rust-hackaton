fn main() {
    let closure = |slice : &[i32]| -> (i32, i32) {
        (slice[0], slice[slice.len() - 1])
    };
  
    println!("{:?}", closure(&[1, 2, 3, 4, 5, 6]));
  }
  