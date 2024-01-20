fn print_slices_element(slice: &[String]) {
    for sl in slice {
        println!("{}", sl);
    }
}

fn main() {
  print_slices_element(&["Thomas".to_string(), "Nassim".to_string(), "Guillaume".to_string()]);
}
