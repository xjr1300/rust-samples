use anyhow::{anyhow, bail, Context as _};

/// `main`関数は、[std::process::Termination](https://doc.rust-lang.org/std/process/trait.Termination.html)トレイトを実装した任意の型を返す必要がある。
/// [anyhow::Result]()は、`std::process::Termination`を実装しているため、`main`関数の戻り値の型として利用できる。
fn main() -> anyhow::Result<()> {
    let result = baz();
    println!("--- {{:?}} ---");
    println!("{:?}", result);
    println!();

    println!("--- {{:#?}} ---");
    println!("{:#?}", result);
    println!();

    let value = Some(42);
    println!("Some(42) to result: {:?}", option_to_result(value));
    println!("None to result: {:?}", option_to_result(None));
    println!();

    println!("validate(42): {:?}", validate(42));
    println!("validate(-1): {:?}", validate(-1));
    println!("improved validate(-1): {:?}", improved_validate(-1));

    Ok(())
}

#[allow(dead_code)]
struct Foo {
    bar: Result<i32, String>,
}

impl Foo {
    #[allow(dead_code)]
    fn new() -> anyhow::Result<Foo> {
        Ok(Foo { bar: Ok(42) })
    }
}

fn foo() -> anyhow::Result<()> {
    Err(anyhow!("foo fun error"))
}

fn bar() -> anyhow::Result<()> {
    foo().context("bar fn error")
}

fn baz() -> anyhow::Result<()> {
    bar().context("baz fn error")
}

/// Optionをanyhow::Resultに変換
fn option_to_result(value: Option<i32>) -> anyhow::Result<i32> {
    value.context("value is None")
}

fn validate(value: i32) -> anyhow::Result<()> {
    if value < 0 {
        return Err(anyhow!("value is negative"));
    }

    Ok(())
}

fn improved_validate(value: i32) -> anyhow::Result<()> {
    if value < 0 {
        // bail!マクロは、return Err(anyhow!(...))の簡略構文
        bail!("value is negative");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// テスト関数の戻り値の型に`anyhow::Result`を使用しない場合、次の通り冗長になる。
    #[test]
    fn test_foo() {
        assert_eq!(Foo::new().unwrap().bar.unwrap(), 42);
    }

    /// テスト関数の戻り値の型に`anyhow::Result`を使用した場合、次の通り少し簡潔になる。
    #[test]
    fn improved_test_foo() -> anyhow::Result<()> {
        assert_eq!(Foo::new()?.bar.unwrap(), 42);

        Ok(())
    }
}
