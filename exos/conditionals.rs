fn evaluate_number(number: i32) {
    if number > 0 {
        println!("positive");
    } else if number < 0 {
        println!("negative");
    } else {
        println!("zero");
    }
}

fn main() {
    evaluate_number(0);
    evaluate_number(5);
    evaluate_number(-5);
}
