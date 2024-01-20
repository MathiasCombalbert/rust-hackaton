
fn is_none(option: Option<i32>) -> bool {
    match option {
        None => true,
        Some(_) => false,

    }
}

fn get_str(slice: &[String], word: &str) -> Option<String> {
    if slice.contains(&word.to_string()) {
        return Some((&word).to_string())
    }
    None
}

fn main() {
    println!("{}", is_none(Some(1)));
    println!("{}", is_none(None));

    println!("{:?}", get_str(&["Hello".to_string(), "World".to_string()], "World"));
}
