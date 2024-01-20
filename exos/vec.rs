fn remove_first_and_last(slice: Vec<i32>) -> Vec<i32> {
    slice.remove(0);
    slice.pop();
    slice
}

fn concat_vec(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut temp = vec1;
    temp.extend(vec2);
    temp
}

fn main() {
    println!("{:?}", remove_first_and_last(vec![1, 2, 3, 4, 5]));
    println!(
        "{:?}",
        concat_vec(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10])
    );
}
