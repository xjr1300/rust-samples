/// ```
/// pub fn as_deref(&self) -> Result<&<T as Deref>::Target, &E>
/// where
///     T: Deref,
/// ```
///
/// Converts from Result<T, E> (or &Result<T, E>) to Result<&<T as Deref>::Target, &E>.
///
/// Coerces the Ok variant of the original Result via Deref and returns the new Result.`
///
/// `Result<T, E>`または`&Result<T, E>`から、`Result<&<T as Deref>::Target, &E>`に変換する。
///
/// `Deref`を経由して元の`Result`の`Ok`バリアントを強制的に変換し、新しい`Result`を返却する。
fn main() {
    // `String`は`Deref`トレイトを実装しているため、`deref`メソッドで`String`を`&str`に変換する。
    let x: Result<String, u32> = Ok(String::from("hello"));
    let y: Result<&str, &u32> = Ok("hello");
    assert_eq!(x.as_deref(), y);

    let x: Result<String, u32> = Err(42);
    let y: Result<&str, &u32> = Err(&42);
    assert_eq!(x.as_deref(), y);
}
