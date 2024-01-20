fn add(x: i32, y : i32) -> i32 {
    return x + y;
}

fn slice_sum(slice: &[i32]) -> i32 {
    let mut sum = 0;

    for nb in slice {
        sum += nb;
    }
    return sum;
}

fn print_modulo(x: i32, y: i32) {
    println!("{}", x%y)
}



fn main() {
    println!("{}", add(1, 2));
    println!("{}", slice_sum(&[1, 2, 3]));
    print_modulo(10, 2);
}
