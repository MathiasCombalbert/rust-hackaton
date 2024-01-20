fn get_element_at<T>(slice: &[T], index: usize) -> Result<&T, String>
{
    if slice.len() < index {
        return Err("Index out of bound".to_string());
    }
    Ok(&slice[index])
}


fn main() {
    println!("{:?}", get_element_at(&["Hello".to_string(), "World".to_string()], 3));
    println!("{:?}", get_element_at(&["Hello".to_string(), "World".to_string()], 1));
}
