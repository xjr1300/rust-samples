/// ```
/// pub const fn as_ref(&self) -> Result<&T, &E>
/// ```
/// Converts from &Result<T, E> to Result<&T, &E>.
///
/// Produces a new Result, containing a reference into the original, leaving the original in place.
///
/// `&Result<T, E>から`Result<&T, &E>`に変換する。
///
/// `Result`の参照を`Result`がラップするインスタンスの参照に変換した`Result`を返却する。
/// Examples
/// let x: Result<u32, &str> = Ok(2);
/// assert_eq!(x.as_ref(), Ok(&2));
///
/// let x: Result<u32, &str> = Err("Error");
/// assert_eq!(x.as_ref(), Err(&"Error"));
fn main() {
    let x: Result<u32, &str> = Ok(2);
    let y: Result<&u32, &&str> = Ok(&2);
    assert_eq!(x.as_ref(), y);

    let x: Result<u32, String> = Err(String::from("hello"));
    let s = String::from("hello");
    let y: Result<&u32, &String> = Err(&s);
    assert_eq!(x.as_ref(), y);
}
