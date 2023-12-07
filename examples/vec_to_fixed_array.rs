use std::convert::TryInto;

fn vec_to_fixed_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], String> {
    v.try_into().map_err(|_| String::from("error"))
}

fn main() {
    let v = vec![0, 1, 2, 3, 4];
    let array = vec_to_fixed_array::<i32, 5>(v);
    assert!(array.is_ok());
    println!("{:?}", array.unwrap());

    let v = vec![0, 1, 2, 3, 4];
    let array = vec_to_fixed_array::<i32, 4>(v);
    assert!(array.is_err());

    let v = vec![0, 1, 2, 3, 4];
    let array = vec_to_fixed_array::<i32, 6>(v);
    assert!(array.is_err());
}
