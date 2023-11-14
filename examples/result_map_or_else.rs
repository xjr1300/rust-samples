/// ```
/// pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
/// where
///     D: FnOnce(E) -> U,
///     F: FnOnce(T) -> U,
/// ```
///
/// Maps a Result<T, E> to U by applying fallback function default to a contained Err value,
/// or function f to a contained Ok value.
///
/// `Err`に含まれている値に対してフォールバック関数`default`を適用するか、`Ok`に含まれている値に対して関数`f`を適用することにより、
/// `Result<T, E>`を`U`に写像する。
///
/// This function can be used to unpack a successful result while handling an error.
///
/// この関数は、成功した結果をアンパックしながらエラーをハンドリングするために使用できる。
///
/// 最初の引数は`Err`に含まれている値に対して適用され、2番目の引数は`Ok`に含まれている値に対して適用される。
///
/// ```rust
/// let k = 21;
///
/// let x : Result<_, &str> = Ok("foo");
/// assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 3);
///
/// let x : Result<&str, _> = Err("bar");
/// assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 42);
/// ```
fn main() {
    let k = 21;

    let x: Result<_, &str> = Ok("foo");
    assert_eq!(x.map_or_else(|_e| k * 2, |v| v.len()), 3);

    let x: Result<&str, _> = Err("foobar");
    assert_eq!(x.map_or_else(|e| e.len(), |v| v.len()), 6);
}
