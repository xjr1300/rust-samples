/// ```
/// pub fn transpose(self) -> Option<Result<T, E>>
/// ```
///
/// Transposes a Result of an Option into an Option of a Result.
///
/// `Result`の`Option`を`Option`の`Result`に入れ替える。
///
/// Ok(None) will be mapped to None. Ok(Some(_)) and Err(_) will be mapped to Some(Ok(_)) and Some(Err(_)).
///
/// `Ok(None)`は`None`に写像される。
/// `Ok(Some(_))`と`Err(_)`は、`Some(Ok(_))`と`Some(Err(_))`に写像される。
///
/// ```rust
/// #[derive(Debug, Eq, PartialEq)]
/// struct SomeErr;
//;
/// let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
/// let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
/// assert_eq!(x.transpose(), y);
/// ```
#[derive(Debug, Eq, PartialEq)]
struct SomeError;

fn main() {
    let x: Result<Option<i32>, SomeError> = Ok(Some(5));
    let y: Option<Result<i32, SomeError>> = Some(Ok(5));
    assert_eq!(x.transpose(), y);
}
