/// ```
/// pub fn map_err<F, O>(self, op: O) -> Result<T, F>
/// where
///     O: FnOnce(E) -> F,
/// ```
///
/// Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value, leaving an Ok value untouched.
///
/// `Err`に含まれている値に対して関数を適用することにより、`Result<T, E>`を`Result<T, F>`に写像して、`Ok`の値は触らずにそのままにする。
///
/// This function can be used to pass through a successful result while handling an error.
///
/// この関数は成功した結果をそのまま渡しながら、エラーをハンドリングするために使用できる。
///
/// ```rust
/// fn stringify(x: u32) -> String { format!("error code: {x}") }
///
/// let x: Result<u32, u32> = Ok(2);
/// assert_eq!(x.map_err(stringify), Ok(2));
///
/// let x: Result<u32, u32> = Err(13);
/// assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
/// ```
fn main() {
    let x: Result<u32, u32> = Ok(41);
    assert_eq!(x.map_err(stringify), Ok(41));

    let x: Result<u32, u32> = Err(41);
    assert_eq!(x.map_err(stringify), Err(String::from("error code: 41")));
}

fn stringify(x: u32) -> String {
    format!("error code: {x}")
}
