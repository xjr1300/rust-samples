//! 可変参照から普遍参照を作成する例
fn main() {
    let mut text = String::from("Hello world!");
    // 可変参照を作成
    let text_mut_ref = &mut text;
    text_mut_ref.push_str("\nGood morning!");
    // 可変参照から不変参照を作成
    // text_refの型が、`&&mut String`ではなく`&String`であることに注意
    // 上記で示した不変参照のキャストは暗黙的に行われる
    let text_ref: &String = text_mut_ref;
    println!("{}", text_ref);

    // // 不変参照が有効なときに、可変参照を使用して文字列の更新を試行
    // // error[E0502]: cannot borrow `*text_mut_ref` as mutable because it is also borrowed as immutable
    // //   --> examples/ref_mut_to_ref.rs:12:5
    // //   |
    // // 8  |     let text_ref: &String = text_mut_ref;
    // //   |                             ------------ immutable borrow occurs here
    // // ...
    // // 12 |     text_mut_ref.push_str("\nGoodby!");
    // //   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
    // // 13 |     println!("{}", text_ref);
    // //   |                    -------- immutable borrow later used here
    // text_mut_ref.push_str("\nGoodby!");
    // text.push_str("\nGoodby!");

    // 不変参照が有効なときでも、可変参照を介して変更する操作以外は可能
    println!("{}", text_mut_ref);
    println!("{}", text_ref);
}
