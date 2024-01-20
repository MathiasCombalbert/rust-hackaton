fn element_at(slice: &[i32], index: usize) -> i32 {
    return slice[index];
}

fn slices_len(slice: &[i32]) -> usize {
    return slice.len();
}

fn main() {
  let numbers = [-9, 1, -2, 7, 1, 4, 6, -2, 7, -9];

  println!("{:?}", slices_len(&numbers));
  println!("{:?}", element_at(&numbers, 1));
}

