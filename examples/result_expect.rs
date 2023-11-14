/// ```
/// pub fn expect(self, msg: &str) -> T
/// where
///     E: Debug,
/// ```
///
/// Returns the contained Ok value, consuming the self value.
///
/// `Ok`に格納された値を返却して、自身の値を消費する。
///
/// Because this function may panic, its use is generally discouraged.
/// Instead, prefer to use pattern matching and handle the Err case explicitly, or call unwrap_or, unwrap_or_else,
/// or unwrap_or_default.
///
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
/// ```rust
/// let x: Result<u32, &str> = Err("emergency failure");
/// x.expect("Testing expect"); // panics with `Testing expect: emergency failure`
/// ```
///
/// ## Recommended Message Style
///
/// We recommend that expect messages are used to describe the reason you expect the Result should be Ok.
///
/// `Result`が`Ok`であると予想した理由を説明する`expect`メッセージを推奨する。
///
/// ```rust
/// let path = std::env::var("IMPORTANT_PATH")
/// .expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
/// ```
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
fn main() {
    let spam =
        std::env::var("SPAM").expect("`SPAM`変数が環境変数に設定されていなければさなりません。");
    println!("spam: {spam}");
}
