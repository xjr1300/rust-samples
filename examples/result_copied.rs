/// ```
/// pub fn copied(self) -> Result<T, E>
/// where
///     T: Copy,
/// ```
///
/// Maps a Result<&T, E> to a Result<T, E> by copying the contents of the Ok part.
///
/// `Ok`部分の内容をコピーすることにより、`Result<&T, E>`から`Result<T, E>`への写像を作成する。
///
/// ```rust
/// let val = 12;
/// let x: Result<&i32, i32> = Ok(&val);
/// assert_eq!(x, Ok(&12));
/// let copied = x.copied();
/// assert_eq!(copied, Ok(12));
/// ```
///
/// Maps a Result<&mut T, E> to a Result<T, E> by copying the contents of the Ok part.
///
/// `Ok`部分の内容をコピーすることにより、`Result<&mut T, E>`から`Result<T, E>`への写像を作成する。
///
/// ```
/// let mut val = 12;
/// let x: Result<&mut i32, i32> = Ok(&mut val);
/// assert_eq!(x, Ok(&mut 12));
/// let copied = x.copied();
/// assert_eq!(copied, Ok(12));
/// ```

fn main() {
    let val = 12;
    let x: Result<&i32, i32> = Ok(&val);
    let copied = x.copied();
    let expected: Result<i32, i32> = Ok(12);
    assert_eq!(copied, expected);

    let x: Result<&i32, i32> = Err(41);
    assert_eq!(x.copied(), Err(41));

    let mut val = 12;
    let x: Result<&mut i32, i32> = Ok(&mut val);
    let copied = x.copied();
    let expected: Result<i32, i32> = Ok(12);
    assert_eq!(copied, expected);

    let x: Result<&mut i32, i32> = Err(41);
    assert_eq!(x.copied(), Err(41));
}
