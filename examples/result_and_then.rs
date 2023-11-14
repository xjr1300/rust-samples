/// ```
/// pub fn and_then<U, F>(self, op: F) -> Result<U, E>
/// where
///     F: FnOnce(T) -> Result<U, E>,
/// ```
/// Calls op if the result is Ok, otherwise returns the Err value of self.
///
/// This function can be used for control flow based on Result values.
///
/// `result`が`Ok`場合は`op`を呼び出し、そうでない場合は`self`の`Err`を返却する。
///
/// この関数は`Result`の値に基づいた制御フローに使用できる。
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("my error")]
struct MyError;

#[derive(Debug, Clone, thiserror::Error)]
#[error("other error")]
struct OtherError;

fn callable(value: i32) -> Result<i32, MyError> {
    Ok(value + 1)
}

fn main() {
    let x: Result<i32, MyError> = Ok(2);
    let x = x.and_then(callable);
    let expected: Result<i32, MyError> = Ok(3);
    assert_eq!(expected, x);

    // 次は成功値の型がu32であり、callable関数の仮引数の型がi32であるためコンパイルエラー
    // let x: Result<u32, MyError> = Ok(2);
    // let x = x.and_then(callable);

    // 次は失敗値の型がOtherErrorであり、callable関数が返却する失敗値の型がMyErrorであるためコンパイルエラー
    // let x: Result<i32, OtherError> = Ok(2);
    // let x = x.and_then(callable);
}
