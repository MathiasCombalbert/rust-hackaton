fn print_counter(c : &u32) {
    println!("{}", c);
}

fn increment_counter(c : &mut u32) {
    *c += 1;
}


fn main() {
    let mut counter = 0u32;
  
    while counter < 5 {
      print_counter(&counter);
      increment_counter(&mut counter);
    }
  }
  