/// ```
/// pub fn cloned(self) -> Result<T, E>
/// where
///     T: Clone,
/// ```
///
/// Maps a Result<&T, E> to a Result<T, E> by cloning the contents of the Ok part.
///
/// `Ok`部分の内容を複製することにより、`Result<&T, E>`から`Result<T, E>`への写像を作成する。
///
/// ```rust
/// let val = 12;
/// let x: Result<&i32, i32> = Ok(&val);
/// assert_eq!(x, Ok(&12));
/// let cloned = x.cloned();
/// assert_eq!(cloned, Ok(12));
/// ```
///
/// Maps a Result<&mut T, E> to a Result<T, E> by cloning the contents of the Ok part.
///
/// `Ok`部分の内容を複製することにより、`Result<&mut T, E>`から`Result<T, E>`への写像を作成する。
///
/// ```rust
/// let mut val = 12;
/// let x: Result<&mut i32, i32> = Ok(&mut val);
/// assert_eq!(x, Ok(&mut 12));
/// let cloned = x.cloned();
/// assert_eq!(cloned, Ok(12));
/// ```
fn main() {
    let text = String::from("spam");
    let x: Result<&String, i32> = Ok(&text);
    let cloned = x.cloned();
    let expected: Result<String, i32> = Ok(String::from("spam"));
    assert_eq!(cloned, expected);

    let x: Result<&String, i32> = Err(41);
    let cloned = x.cloned();
    assert_eq!(cloned, Err(41));

    let mut text = String::from("spam");
    let x: Result<&mut String, i32> = Ok(&mut text);
    let cloned = x.cloned();
    let expected: Result<String, i32> = Ok(String::from("spam"));
    assert_eq!(cloned, expected);

    let x: Result<&mut String, i32> = Err(41);
    let cloned = x.cloned();
    assert_eq!(cloned, Err(41));
}
