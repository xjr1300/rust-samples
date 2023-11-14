/// ```
/// pub fn as_deref_mut(&mut self) -> Result<&mut <T as Deref>::Target, &mut E>
/// where
///     T: DerefMut,
/// ```
///
/// Converts from Result<T, E> (or &mut Result<T, E>) to Result<&mut <T as DerefMut>::Target, &mut E>.
///
/// Coerces the Ok variant of the original Result via DerefMut and returns the new Result.
///
/// mut `Result<T, E>`または`&mut Result<T, E>`から、`Result<&mut <T as DerefMut>::Target, &mut E>`に変換する。
///
/// `DerefMut`を経由して元の`Result`の`Ok`バリアントを強制的に変換し、新しい`Result`を返却する。
fn main() {
    let mut x: Result<String, u32> = Ok(String::from("hello"));
    let y: Result<String, &mut u32> = Ok(String::from("HELLO"));
    assert_eq!(x.as_deref_mut().map(|x| x.to_uppercase()), y);

    let mut x: Result<String, u32> = Err(42);
    let mut value = 42;
    let y: Result<&mut str, &mut u32> = Err(&mut value);
    assert_eq!(x.as_deref_mut(), y);
}
