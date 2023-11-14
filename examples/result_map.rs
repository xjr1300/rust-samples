/// ```
/// pub fn map<U, F>(self, op: F) -> Result<U, E>
/// where
///     F: FnOnce(T) -> U,
/// ```
///
/// Maps a Result<T, E> to Result<U, E> by applying a function to a contained Ok value, leaving an Err value untouched.
///
/// `Ok`に含まれている値に対して関数を適用することにより、`Result<T, E>`を`Result<U, E>`に写像して、`Err`の値は触らずにそのままにする。
///
/// This function can be used to compose the results of two functions.
///
/// この関数は2つの関数の結果を合成するために使用できる。
///
/// ```rust
/// // Print the numbers on each line of a string multiplied by two.
///
/// let line = "1\n2\n3\n4\n";
///
/// for num in line.lines() {
///     match num.parse::<i32>().map(|i| i * 2) {
///         Ok(n) => println!("{n}"),
///         Err(..) => {}
///     }
/// }
/// ```
fn main() {
    let x: Result<u32, u32> = Ok(41);
    let result = x.map(|x| x.to_string());
    assert_eq!(result, Ok(String::from("41")));

    let x: Result<u32, u32> = Err(41);
    assert_eq!(x.map(|x| x.to_string()), Err(41));
}
