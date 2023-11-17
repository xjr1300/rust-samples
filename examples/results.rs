fn main() {
    // `Result`が`Ok`であるか確認
    is_ok();

    // `Result`が`Ok`で、`Ok`の中身が述語を満たすか確認
    is_ok_and();

    // `Result`が`Err`であるか確認
    is_err();

    // `Result`が`Err`で、`Err`の中身が述語を満たすか確認
    is_err_and();

    // `Result<T>`を`Option<%>`に変換
    // `Ok`の場合は`Some<T>`を、`Err`の場合は`None`を、自分自身を消費して返す
    ok();

    // `Result<T, E>`を`Option<E>`に変換
    // `Ok`の場合は`None`を、`Err`の場合は`Some<E>`を、自分自身を消費して返す
    err();

    // `Result<T, E>`を`Result<&T, &E>`に変換
    // `match`式、`ok()`、`unwrap()`メソッドを使用する前に呼び出して、`Result`の中身の所有権が`Result`から移動させない
    as_ref();

    // `Result<T, E>`を`Result<&mut T, &mut E>`に変換
    // `as_ref()`の可変参照バージョン
    as_mut();

    // `Result<T, E>`を`Result<U, E>`に変換
    // `Ok`の場合に、`map()`の引数に渡した関数を適用して、`Result<U, E>`に変換
    // `Err`の場合は、何もしない
    // `Result<T, E>`を、`Result<U, E>`のように、`Ok`の中身の型を変換する。
    map();

    // `Result<T, E>`を`U`に変換
    // `Ok`の場合は、`Ok`に含まれた値に対して関数を適用した結果を返す
    // `Err｀の場合は、デフォルト値を返す
    map_or();

    // `Result<T, E>`を`U`に変換
    // `Ok`の場合は、`Ok`の中身に関数`f`を適用した結果を返す
    // `Err`の場合は、`Err`の中身に関数`default`を適用した結果を返す
    map_or_else();

    // `Result<T, E>`を`Result<T, F>`に変換
    // `Ok`の場合は、そのまま返す
    // `Err`の場合は、`Err`に含まれている値に対して関数を適用して、異なるエラー型を返す
    map_err();

    // `Result<T, E>`または`&Result<T, E>`を`Result<&<T as Deref>::Target, &E>`に変換
    // `Ok`の場合は、`Ok`の中身に`Deref`トレイトの実装を適用して、`&<T as Deref>::Target`を返す
    // `Err`の場合は、`Err`の中身の参照を返す
    // `Deref`を経由して元の`Result`の`Ok`バリアントを強制的に変換し、新しい`Result`を返す。
    as_deref();

    // `mut Result<T, E>`または`&mut Result<T, E>`を`Result<&mut <T as DerefMut>::Target, &mut E>`に変換
    // `Ok`の場合は、`Ok`の中身に`DerefMut`トレイトの実装を適用して、`&mut <T as DerefMut>::Target`を返す
    // `Err`の場合は、`Err`の中身の参照を返す
    // `DerefMut`を経由して元の`Result`の`Ok`バリアントを強制的に変換し、新しい`Result`を返す。
    // `as_deref()`の可変参照バージョン
    as_deref_mut();

    // イテレーターを返す
    // `Ok`の場合は、その中身を返し、次に`next()`が呼ばれた場合は`None`を返す
    // `Err`の場合は、`None`を返す
    // コレクションの内容を`map()`などで加工する場合、`map`で適用する関数が`Result`を返すとき、
    // `Ok`の場合はいてレーションが継続して、`Err`の場合はイテレーションを中断してエラーを返すことができる。
    iter();

    // 可変なイテレーターを返す
    // `iter()`の可変参照バージョン
    iter_mut();

    // `Result`が`Err`の場合にパニックする
    // `Ok`の場合は、`Ok`の中身を返す
    // `Err`の場合は、引数に渡したメッセージを表示してパニックする
    // expect();

    // `Result`の`Ok`の中身を返す
    // `Err`の場合は、パニックする
    unwrap();

    // `Result`の`Ok`の中身、または`Ok`の中身の型のデフォルト値を返す
    // `Ok`の場合は、`Ok`の中身を返す
    // `Err`の場合は、`Ok`の中身の型のデフォルト値を返す
    // `Result<i32, &str>`の場合、`Err`の場合は`0_i32`を返す
    unwrap_or_default();

    // `Ok`の場合は、メッセージと`Ok`の中身の値を出力してパニックする
    // `Err`の場合は、`Err`の中身の値を返して、自身自信を消費する
    // `expect()`のエラーバージョン
    expect_err();

    // `Ok`の場合は、パニックする
    // `Err`の場合は、`Err`の中身の値を返して、自分自身を消費する
    unwrap_err();

    // `Result<T, E>`を`Result<U, E>`に変換する
    // `Ok`の場合は、引数にし指定した`Result<U, E>`を返す
    // `Err`の場合は、元のエラーを返す
    and();

    // 引数の関数を適用して`Result<T, E>`を`Result<U, E>`に変換する
    // `Ok`の場合は、`Ok`の中身の値に引数で指定した関数を適用した結果を返す
    // `Err`の場合は、元のエラーを返す
    and_then();

    // `Result<T, E>`を`Result<T, F>`に変換する
    // `Ok`の場合は、`Ok`を返す
    // `Err`の場合は、引数に指定した`Result<T, F>`を返す
    or();

    // `Ok`の場合は、`Ok`を返す
    // `Err`の場合は、引数に指定した関数を適用して`Result<T, E>`を返す
    // `Result<T, F>`を`Result<T, F>`で返す
    or_else();

    // `Ok`の場合は、`Ok`の中身を返す
    // `Err`の場合は、引数に指定したデフォルト値を返す
    unwrap_or();

    // `Ok`の場合は、`Ok`の中身を返す
    // `Err`の場合は、引数に指定した関数の戻り値を返す
    unwrap_or_else();

    // アンセーフ
    // `Ok`の場合は、`Ok`の中身を返す
    // `Err`の場合は、未定義動作
    unwrap_unchecked();

    // アンセーフ
    // `Ok`の場合は、未定義動作
    // `Err`の場合は、`Err`の中身を返す
    unwrap_err_unchecked();

    // `Result<&T, E>`または`Result<&mut T, E>`を`Result<T, E>`に変換する
    // `T: Clone + Copy`バージョン
    copied();

    // `Result<&T, E>`または`Result<&mut T, E>`を`Result<T, E>`に変換する
    // `T: Clone`バージョン
    cloned();

    // `Result<Option<T>, E>`を`Option<Result<T, E>>`に変換する
    transpose();
}

/// ```
/// pub const fn is_ok(&self) -> bool
/// ```
///
/// Returns true if the result is Ok.
///
/// `Result`が`Ok`であれば、`true`を返す。
/// `Result`が`Err`であれば、`false`を返す。
/// ```
fn is_ok() {
    let x: Result<i32, &str> = Ok(41);
    assert!(x.is_ok());
    let x: Result<i32, &str> = Err("Some error message");
    assert!(x.is_err());
}

/// ```
/// pub fn is_ok_and(self, f: impl FnOnce(T) -> bool) -> bool
/// ```
///
/// Returns true if the result is Ok and the value inside of it matches a predicate.
///
/// `Result`が`Ok`であり、`Ok`の中身の値が述語を満たす場合は、`true`を返す。
/// `Result`が`Err`であるか、`Ok`であってもその中身が述語を満たさない場合は、`false`を返す。
///
/// > `Result`が`Ok`で、さらに`Ok`の中身に対して、追加の試験をしたい場合に利用できそう。
fn is_ok_and() {
    // `Result`が`Ok`であり、その中身が述語を満たす場合
    let x: Result<u32, &str> = Ok(41);
    let result = x.is_ok_and(|x| x == 41);
    assert!(result);

    // `Result`が`Ok`であり、その中身が述語を満たさない場合
    let x: Result<u32, &str> = Ok(41);
    let result = x.is_ok_and(|x| x != 41);
    assert!(!result);

    // `Result`が`Err`の場合
    let x: Result<u32, &str> = Err("spam");
    let result = x.is_ok_and(|x| x == 41);
    assert!(!result);
}

/// ```
/// pub const fn is_err(&self) -> bool
/// ```
///
/// Returns true if the result is Err.
///
/// `Result`が`Err`の場合は、`true`を返す。
/// `Result`が`Ok`の場合は、`false`を返す。
fn is_err() {
    let x: Result<i32, &str> = Ok(41);
    assert!(x.is_ok());
    let x: Result<i32, &str> = Err("Some error message");
    assert!(x.is_err());
}

/// ```
/// pub fn is_err_and(self, f: impl FnOnce(E) -> bool) -> bool
/// ```
///
/// Returns true if the result is Err and the value inside of it matches a predicate.
///
/// `Result`が`Err`であり、`Err`の中身の値が述語を満たす場合は、`true`を返す。
/// このメソッドは自身を消費する。
///
/// > `Result`が`Err`で、さらに`Err`の中身に対して、追加の試験をしたい場合に利用できそう。
fn is_err_and() {
    use std::io::{Error, ErrorKind};
    // `Result`が`Err`で、その中身が述語を満たす場合
    let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
    let result = x.is_err_and(|x| x.kind() == ErrorKind::NotFound);
    assert!(result);

    // `Result`が`Err`で、その中身が述語を満たす場合
    let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
    let result = x.is_err_and(|x| x.kind() == ErrorKind::PermissionDenied);
    assert!(!result);

    // `Result`が`Ok`の場合
    let x: Result<u32, Error> = Ok(41);
    let result = x.is_err_and(|x| x.kind() == ErrorKind::NotFound);
    assert!(!result);
}

/// ```
/// pub fn ok(self) -> Option<T>
/// ```
///
/// Converts from Result<T, E> to Option<T>.
///
/// `Result<T, E>`を`Option<T>`に変換する。
///
/// Converts self into an Option<T>, consuming self, and discarding the error, if any.
///
/// `Ok<T>`であれば、自分自身を消費して`Some<T>`を返す。
/// `Err<E>`であれば、自分自身を消費して`None`を返す。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// assert_eq!(x.ok(), Some(2));
///
/// let x: Result<u32, &str> = Err("Nothing here");
/// assert_eq!(x.ok(), None);
/// ```
fn ok() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("spam");
    assert_eq!(x.ok(), None);
}

/// ```
/// pub fn err(self) -> Option<E>
/// ```
/// Converts from Result<T, E> to Option<E>.
///
/// Converts self into an Option<E>, consuming self, and discarding the success value, if any.
///
/// `Result<T, E>`から`Option<E>`に変換する。
///
/// `Result`が`Ok`の場合は、自信を消費して`None`を返す。
/// `Result`が`Err`の場合は、自信を消費して`Some<E>`を返す。
fn err() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.err(), None);
    let x: Result<u32, &str> = Err("spam");
    assert_eq!(x.err(), Some("spam"));
}

/// ```
/// pub const fn as_ref(&self) -> Result<&T, &E>
/// ```
/// Converts from &Result<T, E> to Result<&T, &E>.
///
/// Produces a new Result, containing a reference into the original, leaving the original in place.
///
/// `&Result<T, E>から`Result<&T, &E>`に変換する。
///
/// `Result`の参照を`Result`がラップするインスタンスの参照に変換した`Result`を返す。
///
/// > `Result`に対して`match`, `unwrap`, `ok`, `err`などを呼び出すと、`Result`から中身の所有権が移動する。
/// > `Result`の中身の所有権を移動したくない場合に、`as_ref`を使用する。
#[allow(clippy::unnecessary_literal_unwrap)]
fn as_ref() {
    let x: Result<u32, &str> = Ok(2);
    let y: Result<&u32, &&str> = Ok(&2);
    assert_eq!(x.as_ref(), y);

    let x: Result<u32, String> = Err(String::from("hello"));
    let s = String::from("hello");
    let y: Result<&u32, &String> = Err(&s);
    assert_eq!(x.as_ref(), y);

    let x: Result<String, &str> = Ok(String::from("hello"));
    if x.ok() == Some(String::from("hello")) {
        // 次は、`x.ok()`で、xから中身の所有権が移動しているためエラー
        // assert_eq!("hello", x.unwrap());
    }

    let x: Result<String, &str> = Ok(String::from("hello"));
    if x.as_ref().ok() == Some(&String::from("hello")) {
        // 次は、上の`x.as_ref()`で、`Result<String, &str>`を`Result<&String, &&str>`に変換している。
        // そして、`ok()`で、それを`Option<&String>`に変換している。
        // `Result`の`Ok<&String>`と`Option<&String>`は、元の`Ok<String>`の中身の参照であるため、
        // 元の`Ok<String>`の中身の所有権は移動されていなし。
        assert_eq!("hello", x.unwrap());
    }
}

/// ```
/// pub fn as_mut(&mut self) -> Result<&mut T, &mut E>
/// ```
///
/// Converts from &mut Result<T, E> to Result<&mut T, &mut E>.
///
/// `&mut Result<T, E>`から`Result<&mut T, &mut E>`に変換する。
/// `as_ref()`の可変参照バージョンである。
fn as_mut() {
    let mut x: Result<String, String> = Ok(String::from("hello"));

    // 次は、`match x`で、`x`の所有権が`match`式の内部に移動しているため、`assert_eq!`でエラーが発生する。
    // match x {
    //     Ok(mut o) => o.push_str(" world!"),
    //     Err(mut e) => e.push_str(" to hell!"),
    // };
    // assert_eq!(x, Ok(String::from("hello world!")));

    // `as_mut()`を使用して、`x`から所有権が移動させずに可変にする。
    match x.as_mut() {
        Ok(o) => o.push_str(" world!"),
        Err(e) => e.push_str(" to hell!"),
    };
    // xの所有権が移動していないため、xを利用できる。
    assert_eq!(x, Ok(String::from("hello world!")));

    let mut y: Result<String, String> = Err(String::from("welcome"));
    match y.as_mut() {
        Ok(o) => o.push_str(" world!"),
        Err(e) => e.push_str(" to hell!"),
    };
    assert_eq!(y, Err(String::from("welcome to hell!")));
}

/// ```
/// pub fn map<U, F>(self, op: F) -> Result<U, E>
/// where
///     F: FnOnce(T) -> U,
/// ```
///
/// Maps a Result<T, E> to Result<U, E> by applying a function to a contained Ok value, leaving an Err value untouched.
///
/// `Ok`に含まれている値に対して関数を適用することにより、`Result<T, E>`を`Result<U, E>`に写像する。
/// `Err`の場合は、何もしない。
///
/// > `Result`が`Ok`で、さらに`Ok`の中身を加工する場合に利用できそう。
fn map() {
    // `map()`を利用して、`Result<u32, u32>`を`Result<String, u32>`に変換
    let x: Result<u32, u32> = Ok(41);
    let result = x.map(|x| x.to_string());
    assert_eq!(result, Ok(String::from("41")));

    // `Err`の場合は、何もしない
    let x: Result<u32, u32> = Err(41);
    assert_eq!(x.map(|x| x.to_string()), Err(41));
}

/// ```
/// pub fn map_or<U, F>(self, default: U, f: F) -> U
/// where
///     F: FnOnce(T) -> U,
/// ```
///
/// Returns the provided default (if Err), or applies a function to the contained value (if Ok),
///
/// `Ok`の場合は、`Ok`に含まれた値に対して関数を適用した結果を返す。
/// `Err｀の場合は、デフォルト値を返す。
///
/// Arguments passed to map_or are eagerly evaluated; if you are passing the result of a function call,
/// it is recommended to use map_or_else, which is lazily evaluated.
///
/// `map_or`に渡される値は即座に評価される。
/// もし関数呼び出しの結果を渡す場合は、遅延評価される`map_or_else`を使用することを推奨する。
fn map_or() {
    // `Result<&str, &str>`に対して、`map_or()`を使用して、`usize`に変換
    let x: Result<&str, &str> = Ok("foo");
    let x_len: usize = x.map_or(0, |v| v.len());
    assert_eq!(x_len, 3);

    let y: Result<&str, &str> = Err("foo");
    let y_len: usize = y.map_or(0, |v| v.len());
    assert_eq!(y_len, 0);
}

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
/// `Result<T, E>`を`U`に写像する。
/// `Ok`の場合は、`Ok`の中身に関数`f`を適用した結果を返す。
/// `Err`の場合は、`Err`の中身に関数`default`を適用した結果を返す。
fn map_or_else() {
    let x: Result<&str, &str> = Ok("foo");
    let xx: usize = x.map_or_else(|e| e.len() * 2, |o| o.len());
    assert_eq!(3, xx);

    let y: Result<&str, &str> = Err("foo");
    let yy: usize = y.map_or_else(|e| e.len() * 2, |o| o.len());
    assert_eq!(6, yy);
}

/// ```
/// pub fn map_err<F, O>(self, op: O) -> Result<T, F>
/// where
///     O: FnOnce(E) -> F,
/// ```
///
/// Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value, leaving an Ok value untouched.
///
/// `Result<T, E>`を`Result<T, F>`に変換する。
/// `Ok`の場合は、そのまま返す。
/// `Err`の場合は、`Err`に含まれている値に対して関数を適用して、異なるエラー型を返す。
fn map_err() {
    let f = |e: u32| format!("error code: {e}");

    let x: Result<u32, u32> = Ok(41);
    let xx: Result<u32, String> = x.map_err(f);
    assert_eq!(Ok(41), xx);

    let y: Result<u32, u32> = Err(41);
    let yy: Result<u32, String> = y.map_err(f);
    assert_eq!("error code: 41", yy.err().unwrap());
}

/// ```
/// pub fn as_deref(&self) -> Result<&<T as Deref>::Target, &E>
/// where
///     T: Deref,
/// ```
///
/// Converts from Result<T, E> (or &Result<T, E>) to Result<&<T as Deref>::Target, &E>.
/// Coerces the Ok variant of the original Result via Deref and returns the new Result.`
///
/// `Result<T, E>`または`&Result<T, E>`から、`Ok`の中身に対して`Deref`トレイトの実装を適用して、
/// `Result<T, E>`または`&Result<T, E>`から、`Result<&<T as Deref>::Target, &E>`に変換する。
///
/// `Deref`を経由して元の`Result`の`Ok`バリアントを強制的に変換し、新しい`Result`を返す。
fn as_deref() {
    // `String`は`Deref`トレイトを実装しているため、`deref`メソッドで`String`を`&str`に変換する。
    let x: Result<String, u32> = Ok(String::from("hello"));
    let y: Result<&str, &u32> = Ok("hello");
    // `as_deref()`を使用して、`Result<String, u32>`を`Result<&str, &u32>`に変換
    assert_eq!(x.as_deref(), y);

    // `Result`が`Err`であるため、単に`Err`の中身を参照に変換
    let x: Result<String, u32> = Err(42);
    let y: Result<&str, &u32> = Err(&42);
    assert_eq!(x.as_deref(), y);
}

/// ```
/// pub fn as_deref_mut(&mut self) -> Result<&mut <T as Deref>::Target, &mut E>
/// where
///     T: DerefMut,
/// ```
///
/// Converts from Result<T, E> (or &mut Result<T, E>) to Result<&mut <T as DerefMut>::Target, &mut E>.
/// Coerces the Ok variant of the original Result via DerefMut and returns the new Result.
///
/// `mut Result<T, E>`または`&mut Result<T, E>`から、`Result<&mut <T as DerefMut>::Target, &mut E>`に変換する。
/// `DerefMut`を経由して元の`Result`の`Ok`バリアントを強制的に変換し、新しい`Result`を返す。
fn as_deref_mut() {
    let mut x: Result<String, u32> = Ok(String::from("hello"));
    let y: Result<String, &mut u32> = Ok(String::from("HELLO"));
    assert_eq!(x.as_deref_mut().map(|x| x.to_uppercase()), y);

    let mut x: Result<String, u32> = Err(42);
    let mut value = 42;
    let y: Result<&mut str, &mut u32> = Err(&mut value);
    assert_eq!(x.as_deref_mut(), y);
}

/// ```
/// pub fn iter(&self) -> Iter<'_, T>
/// ```
///
/// Returns an iterator over the possibly contained value.
/// The iterator yields one value if the result is Result::Ok, otherwise none.
///
/// 含まれた可能性のある値に対するイテレーターを返す。
/// そのイテレーターは`Result::Ok`の場合は1つの値を返して、そうでない場合は`None`を返す。
///
/// > `std::iter::Map`は`Iterator`を実装している。
/// > 例えば、コレクションを順番に走査するときに、`map()`を使用した時にエラーが発生した時、その走査を中断してエラーを返すことができる。
#[allow(clippy::useless_vec)]
fn iter() {
    let x: Result<u32, &str> = Ok(41);
    assert_eq!(x.iter().next(), Some(&41));
    let x: Result<u32, &str> = Err("spam");
    assert_eq!(x.iter().next(), None);

    fn option_str_to_string(s: Option<&str>) -> Result<String, String> {
        match s {
            Some(s) => Ok(s.to_string()),
            None => Err("Number not found".to_string()),
        }
    }

    let words = vec![Some("alpha"), Some("beta"), Some("gamma"), Some("delta")];
    let words: Result<Vec<String>, String> =
        words.iter().map(|w| option_str_to_string(*w)).collect();
    assert!(words.is_ok());
    assert_eq!(words.unwrap(), vec!["alpha", "beta", "gamma", "delta"]);

    let words = vec![Some("alpha"), Some("beta"), None, Some("delta")];
    let words: Result<Vec<String>, String> =
        words.iter().map(|w| option_str_to_string(*w)).collect();
    assert!(words.is_err());
    assert_eq!(words.err().unwrap(), "Number not found");
}

/// ```
/// pub fn iter_mut(&mut self) -> IterMut<'_, T>
/// ```
///
/// Returns a mutable iterator over the possibly contained value.
/// The iterator yields one value if the result is Result::Ok, otherwise none.
///
/// 含まれた可能性のある値に対する可変イテレーターを返す。
/// そのイテレーターは`Result::Ok`の場合は1つの値を返して、そうでない場合は`None`を返す。
#[allow(clippy::single_match)]
fn iter_mut() {
    let mut x: Result<u32, &str> = Ok(41);
    match x.iter_mut().next() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(x, Ok(42));

    let mut x: Result<u32, &str> = Err("spam");
    assert_eq!(x.iter_mut().next(), None);

    let x: Result<u32, &str> = Ok(41);
    assert_eq!(x.iter().next(), Some(&41));
    let x: Result<u32, &str> = Err("spam");
    assert_eq!(x.iter().next(), None);

    fn push_str_to_option_str(s: Option<&mut String>) -> Result<&mut String, String> {
        match s {
            Some(s) => {
                s.push('A');
                Ok(s)
            }
            None => Err("Number not found".to_string()),
        }
    }

    let mut words = vec![
        Some(String::from("alpha")),
        Some(String::from("beta")),
        Some(String::from("gamma")),
        Some(String::from("delta")),
    ];
    let words: Result<Vec<&mut String>, String> = words
        .iter_mut()
        .map(|w| push_str_to_option_str(w.as_mut()))
        .collect();
    assert!(words.is_ok());
    assert_eq!(words.unwrap(), vec!["alphaA", "betaA", "gammaA", "deltaA"]);

    let mut words = vec![
        Some(String::from("alpha")),
        Some(String::from("beta")),
        None,
        Some(String::from("delta")),
    ];
    let words: Result<Vec<&mut String>, String> = words
        .iter_mut()
        .map(|w| push_str_to_option_str(w.as_mut()))
        .collect();
    assert!(words.is_err());
    assert_eq!(words.err().unwrap(), "Number not found");
}

/// ```
/// pub fn expect(self, msg: &str) -> T
/// where
///     E: Debug,
/// ```
///
/// Returns the contained Ok value, consuming the self value.
/// Because this function may panic, its use is generally discouraged.
/// Instead, prefer to use pattern matching and handle the Err case explicitly, or call unwrap_or, unwrap_or_else,
/// or unwrap_or_default.
///
/// `Ok`に格納された値を返して、自身の値を消費する。
/// この関数はパニックする可能性があるため、一般的には使用を避けるべきである。
/// 代わりに、パターンマッチングを使用して、明示的に`Err`の場合を処理するか、`unwrap_or`、`unwrap_or_else`、`unwrap_or_default`を
/// 呼び出した方が良い。
///
/// ## Panics
///
/// Panics if the value is an Err, with a panic message including the passed message, and the content of the Err.
///
/// もじ値が`Err`の場合は、パニックメッセージと**`Err`の内容を含むパニックメッセージ**と一緒にパニックする。
///
/// ## Recommended Message Style
///
/// We recommend that expect messages are used to describe the reason you expect the Result should be Ok.
///
/// `expect`のメッセージは、`Result`が`Ok`であるべきと予期した理由を説明するために使用されることを推奨する。
///
/// Hint: If you’re having trouble remembering how to phrase expect error messages remember to focus
/// on the word “should” as in “env variable should be set by blah” or “the given binary should be available
/// and executable by the current user”.
///
/// ヒント: `expect`エラーメッセージのフレーズを覚えるのに苦労している場合は、「blahによって環境変数が設定されるべきである」や
/// 「指定されたバイナリは現在のユーザーによって利用可能で実行可能であるべきである」のように「すべき」という単語に注目してください。
///
/// For more detail on expect message styles and the reasoning behind our recommendation
/// please refer to the section on “Common Message Styles” in the std::error module docs.
///
/// `expect`メッセージスタイルと推奨理由の詳細については、std::errorモジュールのドキュメントの「Common Message Styles」のセクションを参照してください。
#[allow(dead_code)]
fn expect() {
    let spam =
        std::env::var("SPAM").expect("`SPAM`変数が環境変数に設定されていなければさなりません。");
    println!("spam: {spam}");
}

/// ```
/// pub fn unwrap(self) -> T
/// where
///     E: Debug,
/// ```
///
/// Returns the contained Ok value, consuming the self value.
/// Because this function may panic, its use is generally discouraged.
/// Instead, prefer to use pattern matching and handle the Err case explicitly, or call unwrap_or, unwrap_or_else, or unwrap_or_default.
///
/// `Ok`に含まれた値を返して、自身の値を消費する。
/// この関数はパニックする可能性があるため、一般的にはその使用は推奨されない。
/// 代わりに、パターンマッチングを使用して、明示的に`Err`ケースを処理するか、`unwrap_or`、`unwrap_or_else`、`unwrap_or_default`を呼び出すことを推奨する。
///
/// ## Panics
///
/// Panics if the value is an Err, with a panic message provided by the Err’s value.
///
/// もし値が`Err`の場合、`Err`の値によって提供されるパニックメッセージとともにパニックする。
#[allow(clippy::unnecessary_literal_unwrap)]
fn unwrap() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);

    // let x: Result<u32, &str> = Err("emergency failure");
    // x.unwrap(); // panic!
}

/// ```
/// pub fn unwrap_or_default(self) -> T
/// where
///     T: Default,
/// ```
///
/// Returns the contained Ok value or a default
///
/// Consumes the self argument then, if Ok, returns the contained value, otherwise if Err, returns the default value for that type.
///
/// `Ok`に含まれた値かデフォルト値を返す。
/// `self`引数を消費して、もし`Ok`の場合は含まれた値を返して、`Err`の場合はその方のデフォルトの値を返す。
fn unwrap_or_default() {
    let input = "1909";
    let expected = 1909;
    let result = input.parse().unwrap_or_default();
    assert_eq!(expected, result);

    let bad_input = "190spam";
    let expected = 0;
    let result = bad_input.parse().unwrap_or_default();
    assert_eq!(expected, result);
}

/// ```
/// pub fn expect_err(self, msg: &str) -> E
/// where
///     T: Debug,
/// ```
///
/// Returns the contained Err value, consuming the self value.
///
/// 含まれている`Err`の値を返して、自身の値を消費する。
///
/// `Ok`の場合は、引数に渡したメッセージとともに、`Ok`の中身の値を出力してパニックする。
/// `Err`の場合は、`Err`の中身の値を返して、自身の値を消費する。
#[allow(clippy::unnecessary_literal_unwrap)]
fn expect_err() {
    // let x: Result<&str, i32> = Ok("ok");
    // x.expect_err("Testing expect_err"); // panics with `Testing expect_err: "ok"`

    let y: Result<&str, i32> = Err(0);
    y.expect_err("Testing expect_err");
}

/// ```
/// pub fn unwrap_err(self) -> E
/// where
///     T: Debug,
/// ```
///
/// Returns the contained Err value, consuming the self value.
///
/// `Err`に含まれた値を返して、自身の値を消費する。
///
/// ## Panics
///
/// Panics if the value is an Ok, with a custom panic message provided by the Ok’s value.
///
/// もし値が`Ok`の場合、`Ok`の値によって提供されるカスタムパニックメッセージと一緒にパニックする。
#[allow(clippy::unnecessary_literal_unwrap)]
fn unwrap_err() {
    let x: Result<u32, &str> = Err("emergency failure");
    assert_eq!(x.unwrap_err(), "emergency failure");

    // let x: Result<u32, &str> = Ok(2);
    // x.unwrap_err(); // panic with `2`
}

/// ```
/// pub fn and<U>(self, res: Result<U, E>) -> Result<U, E>
/// ```
///
/// Returns res if the result is Ok, otherwise returns the Err value of self.
///
/// Arguments passed to and are eagerly evaluated;
/// if you are passing the result of a function call, it is recommended to use and_then, which is lazily evaluated.
///
/// もし結果が`Ok`の場合`res`を返して、そうでなければ`self`の`Err`値を返す。
///
/// 渡された引数は優先に（`eagerly`）評価される。
/// もし、関数呼び出しの結果を足す場合は、遅延評価される`and_then`を使用することを推奨する。
///
/// `self`が`Ok`の場合、`res`を返す。
/// `self`が`Err`の場合、`self`自身の`Err`値を返す。
///
/// `Result<T, E>`を、`Result<U, E>`のように、`Ok`の中身の型を変換する。
fn and() {
    let x: Result<&str, u32> = Ok("ok");
    let y: Result<u32, u32> = Err(0);
    let z: Result<u32, u32> = x.and(y);
    assert_eq!(z, Err(0));

    let x: Result<&str, u32> = Ok("ok");
    let y: Result<u32, u32> = Ok(4);
    let z: Result<u32, u32> = x.and(y);
    assert_eq!(z, Ok(4));

    let x: Result<&str, u32> = Err(0);
    let y: Result<u32, u32> = Ok(4);
    let z: Result<u32, u32> = x.and(y);
    assert_eq!(z, Err(0));

    let x: Result<&str, u32> = Err(0);
    let y: Result<u32, u32> = Err(1);
    let z: Result<u32, u32> = x.and(y);
    assert_eq!(z, Err(0));
}

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
fn and_then() {
    #[derive(Debug, Clone, PartialEq, thiserror::Error)]
    #[error("my error")]
    struct MyError;

    #[derive(Debug, Clone, thiserror::Error)]
    #[error("other error")]
    struct OtherError;

    fn callable(value: i32) -> Result<String, MyError> {
        Ok((value + 1).to_string())
    }

    let x: Result<i32, MyError> = Ok(2);
    let x = x.and_then(callable);
    let expected: Result<String, MyError> = Ok(String::from("3"));
    assert_eq!(expected, x);
}

/// ```
/// pub fn or<F>(self, res: Result<T, F>) -> Result<T, F>
/// ```
///
/// Returns res if the result is Err, otherwise returns the Ok value of self.
/// Arguments passed to or are eagerly evaluated; if you are passing the result of a function call,
/// it is recommended to use or_else, which is lazily evaluated.
///
/// もし`Result`が`Err`の場合は`res`を返却し、そうでない場合は自身の`Ok`の値を返却する。
/// `or`に渡された引数は即時に評価される。もし、関数呼び出しの結果を渡す場合は、遅延評価される`or_else`を使用することを推奨する。
///
/// `Result<T, E>`を、`Result<T, F>`のように、`Err`の中身の型を変換する。
fn or() {
    let x: Result<u32, &str> = Ok(41);
    let y: Result<u32, u32> = Err(0);
    let z: Result<u32, u32> = x.or(y);
    assert_eq!(z, Ok(41));

    let x: Result<u32, &str> = Err("early error");
    let y: Result<u32, u32> = Ok(41);
    let z: Result<u32, u32> = x.or(y);
    assert_eq!(z, Ok(41));

    let x: Result<u32, &str> = Err("early error");
    let y: Result<u32, u32> = Err(0);
    let z: Result<u32, u32> = x.or(y);
    assert_eq!(z, Err(0));

    let x: Result<u32, u32> = Ok(41);
    let y: Result<u32, u32> = Ok(42);
    let z: Result<u32, u32> = x.or(y);
    assert_eq!(z, Ok(41));
}

/// ```
/// pub fn or_else<F, O>(self, op: O) -> Result<T, F>
/// where
///     O: FnOnce(E) -> Result<T, F>,
/// ```
///
/// Calls op if the result is Err, otherwise returns the Ok value of self.
/// This function can be used for control flow based on result values.
///
/// もし`Result`が`Err`の場合は`op`を呼び出し、そうでない場合は自身の`Ok`の値を返却する。
/// この関数は、`Result`の値に基づいたコントロールフローに使用できる。
fn or_else() {
    fn len(s: &str) -> Result<usize, usize> {
        Ok(s.len())
    }

    fn err(s: &str) -> Result<usize, usize> {
        Err(s.len())
    }

    let x: Result<usize, &str> = Ok(2);
    assert_eq!(x.or_else(len), Ok(2));
    let y: Result<usize, &str> = Err("foobar");
    assert_eq!(y.or_else(len), Ok(6));

    let x: Result<usize, &str> = Ok(2);
    assert_eq!(x.or_else(err), Ok(2));
    let y: Result<usize, &str> = Err("foobar");
    assert_eq!(y.or_else(err), Err(6));
}

/// ```
/// pub fn unwrap_or(self, default: T) -> T
/// ```
///
/// Returns the contained Ok value or a provided default.
///
/// `Ok`に含まれた値が、提供されたデフォルト値を返却する。
///
/// Arguments passed to unwrap_or are eagerly evaluated; if you are passing the result of a function call,
/// it is recommended to use unwrap_or_else, which is lazily evaluated.
///
/// `unwrap_or`に渡された引数は即座に評価される。もし関数呼び出しの結果を渡している場合は、遅延評価される`unwrap_or_else`を使用することが推奨される。
#[allow(clippy::unnecessary_literal_unwrap)]
fn unwrap_or() {
    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);

    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), 2);
}

/// ```
/// pub fn unwrap_or_else<F>(self, op: F) -> T
/// where
///     F: FnOnce(E) -> T,
/// ```
///
/// Returns the contained Ok value or computes it from a closure.
///
/// `Ok`に含まれた値か、クロージャーから返された値を返却する。
#[allow(clippy::unnecessary_literal_unwrap)]
fn unwrap_or_else() {
    fn count(x: &str) -> usize {
        x.len()
    }
    assert_eq!(Ok(2).unwrap_or_else(count), 2);
    assert_eq!(Err("spam").unwrap_or_else(count), 4);
}

/// ```
/// pub unsafe fn unwrap_unchecked(self) -> T
/// ```
///
/// Returns the contained Ok value, consuming the self value, without checking that the value is not an Err.
///
/// 値が`Err`であることを確認しないで、`Ok`に含まれた値を返却して、自身を消費する。
///
/// ## Safety
///
/// Calling this method on an Err is undefined behavior.
///
/// `Err`に対するこのメソッドの呼び出しは未定義な動作になる。
#[allow(clippy::unnecessary_literal_unwrap)]
fn unwrap_unchecked() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(unsafe { x.unwrap_unchecked() }, 2);

    // let x: Result<u32, &str> = Err("emergency failure");
    // unsafe {
    //     // Undefined behavior!
    //     x.unwrap_unchecked();
    // }
}

/// ```
/// pub unsafe fn unwrap_err_unchecked(self) -> E
/// ```
///
/// Returns the contained Err value, consuming the self value, without checking that the value is not an Ok.
///
/// `Err`に含まれた値を返却して、自身の値を消費するが、値が`Ok`でないことをチェックしない。
///
/// ## Safety
///
/// Calling this method on an Ok is undefined behavior.
///
/// `Ok`に対してこのメソッドの呼び出しは、未定義な動作になる。
#[allow(clippy::unnecessary_literal_unwrap)]
fn unwrap_err_unchecked() {
    let x: Result<u32, &str> = Err("emergency failure");
    assert_eq!(unsafe { x.unwrap_err_unchecked() }, "emergency failure");

    // let x: Result<u32, &str> = Ok(2);
    // unsafe { x.unwrap_err_unchecked() }; // 未定義な動作
}

/// ```
/// pub fn copied(self) -> Result<T, E>
/// where
///     T: Copy,
/// ```
///
/// Maps a Result<&T, E> to a Result<T, E> by copying the contents of the Ok part.
/// Maps a Result<&mut T, E> to a Result<T, E> by copying the contents of the Ok part.
///
/// `Ok`部分の内容をコピーすることにより、`Result<&T, E>`から`Result<T, E>`への写像を作成する。
/// `Ok`部分の内容をコピーすることにより、`Result<&mut T, E>`から`Result<T, E>`への写像を作成する。
fn copied() {
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
/// Maps a Result<&mut T, E> to a Result<T, E> by cloning the contents of the Ok part.
///
/// `Ok`部分の内容を複製することにより、`Result<&mut T, E>`から`Result<T, E>`への写像を作成する。
fn cloned() {
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

/// ```
/// pub fn transpose(self) -> Option<Result<T, E>>
/// ```
///
/// Transposes a Result of an Option into an Option of a Result.
///
/// transpose: 入れ替える、逆にする、置き換える
///
/// `Result`の`Option`を`Option`の`Result`に入れ替える。
///
/// Ok(None) will be mapped to None. Ok(Some(_)) and Err(_) will be mapped to Some(Ok(_)) and Some(Err(_)).
///
/// `Ok(None)`は`None`に写像される。
/// `Ok(Some(_))`と`Err(_)`は、`Some(Ok(_))`と`Some(Err(_))`に写像される。
fn transpose() {
    #[derive(Debug, Eq, PartialEq)]
    struct SomeError;

    let x: Result<Option<i32>, SomeError> = Ok(Some(5));
    let y: Option<Result<i32, SomeError>> = Some(Ok(5));
    assert_eq!(x.transpose(), y);

    let x: Result<Option<i32>, &str> = Ok(None);
    assert_eq!(x.transpose(), None);

    let x: Result<Option<i32>, &str> = Ok(Some(1));
    assert_eq!(x.transpose(), Some(Ok(1)));

    let x: Result<Option<i32>, &str> = Err("some error");
    assert_eq!(x.transpose(), Some(Err("some error")));
}
