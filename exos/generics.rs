fn last_element<T>(list: &[T]) -> &T
{
    &list[list.len() - 1]
}



fn main() {
    let string_slice = ["Hello".to_string(), "world".to_string()];
    let i32_slice = [1, 2, 3];
    let char_slice = ['a', 'b', 'c'];

    println!("{}", last_element(&string_slice));
    println!("{}", last_element(&i32_slice));
    println!("{}", last_element(&char_slice));
}
