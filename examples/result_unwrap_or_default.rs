/// ```
/// pub fn unwrap_or_default(self) -> T
/// where
///     T: Default,
/// ```
///
/// Returns the contained Ok value or a default
///
/// `Ok`に含まれた値かデフォルト値を返却する。
///
/// Consumes the self argument then, if Ok, returns the contained value, otherwise if Err, returns the default value for that type.
///
/// `self`引数を消費して、もし`Ok`の場合は含まれた値を返却して、`Err`の場合はその方のデフォルトの値を返却する。
///
/// ```rust
/// // Converts a string to an integer, turning poorly-formed strings into 0 (the default value for integers).
/// // parse converts a string to any other type that implements FromStr, returning an Err on error.
/// // 文字列を整数に変換して、不正な文字列を0(整数のデフォルト値)に変換する。
/// // `parse`は文字列を`FromStr`を実装する任意の型に変換して、エラーが発生したときは`Err`を返却する。
/// let good_year_from_input = "1909";
/// let bad_year_from_input = "190blarg";
/// let good_year = good_year_from_input.parse().unwrap_or_default();
/// let bad_year = bad_year_from_input.parse().unwrap_or_default();
///
/// assert_eq!(1909, good_year);
/// assert_eq!(0, bad_year);
/// ```
fn main() {
    let input = "1909";
    let expected = 1909;
    let result = input.parse().unwrap_or_default();
    assert_eq!(expected, result);

    let bad_input = "190spam";
    let expected = 0;
    let result = bad_input.parse().unwrap_or_default();
    assert_eq!(expected, result);
}
