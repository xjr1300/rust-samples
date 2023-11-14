/// ```
/// pub fn and<U>(self, res: Result<U, E>) -> Result<U, E>
/// ```
///
/// Returns res if the result is Ok, otherwise returns the Err value of self.
///
/// Arguments passed to and are eagerly evaluated;
/// if you are passing the result of a function call, it is recommended to use and_then, which is lazily evaluated.
///
/// もし結果が`Ok`の場合`res`を返却して、そうでなければ`self`の`Err`値を返却する。
///
/// 渡された引数は優先に（`eagerly`）評価される。
/// もし、関数呼び出しの結果を足す場合は、遅延評価される`and_then`を使用することを推奨する。
///
/// `self`が`Ok`の場合、`res`を返却する。
/// `self`が`Err`の場合、`self`自身の`Err`値を返却する。
/// ```
fn main() {
    let x: Result<u32, &str> = Ok(2);
    let y: Result<u32, &str> = Err("late error");
    assert_eq!(x.and(y), Err("late error"));

    let x: Result<u32, &str> = Ok(2);
    let y: Result<u32, &str> = Ok(4);
    assert_eq!(x.and(y), Ok(4));

    let x: Result<u32, &str> = Err("early error");
    let y: Result<u32, &str> = Ok(4);
    assert_eq!(x.and(y), Err("early error"));

    let x: Result<u32, &str> = Err("early error");
    let y: Result<u32, &str> = Err("late error");
    assert_eq!(x.and(y), Err("early error"));
}
