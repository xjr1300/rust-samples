fn main() {
    let mut a = String::from("Hello");
    let b = &mut a;
    let c: &mut _ = b; // 再借用（可変参照から新たな可変参照を作成しており、bからcにムーブしていないことに注意）
                       // error[E0502]: cannot borrow `b` as immutable because it is also borrowed as mutable
                       //  --> examples/move_ref_mut.rs:6:20
                       //  |
                       // 4 |     let c: &mut _ = b;
                       //  |                     - mutable borrow occurs here
                       // 5 |     println!("{}", c);
                       // 6 |     println!("{}", b);
                       //  |                    ^ immutable borrow occurs here
                       // 7 |     println!("{}", c);
                       //  |                    - mutable borrow later used here
                       // // println!("{}", c);
    println!("{}", c); // cの寿命が尽きる
    println!("{}", b); // cの寿命が尽きたため、bが有効になる

    let d = b; // bが所有する可変参照は、dにムーブ
    println!("{}", d);
    // //    --> examples/move_ref_mut.rs:21:20
    // //    |
    // // 3  |     let b = &mut a;
    // //    |         - move occurs because `b` has type `&mut String`, which does not implement the `Copy` trait
    // // ...
    // // 19 |     let d = b; // bが所有する可変参照は、dにムーブ
    // //    |             - value moved here
    // // 20 |     println!("{}", d);
    // // 21 |     println!("{}", b); // bが所有していた可変参照はdにムーブしたためエラー
    // //    |                    ^ value borrowed here after move
    // println!("{}", b); // bが所有していた可変参照はdにムーブしたためエラー

    #[allow(clippy::disallowed_names)]
    let mut foo = Foo { a: 42 };

    // 可変参照を取得
    let mut_ref_foo = &mut foo;
    // 可変参照を介して、Fooのフィールドを変更
    // incメソッドは、&mut selfを引数に取る
    // ここでmut_ref_fooが`incメソッドにムーブされると困る
    // よって、Rustは可変参照から新たな可変参照を作成して、メソッドを呼び出す
    // ただし、可変参照は1つしか存在できないため、一時的にmut_ref_fooは無効化される
    mut_ref_foo.inc();
    // ここで、mut_ref_fooが再び有効になる
    println!("{}", mut_ref_foo.a);
}

struct Foo {
    a: i32,
}

impl Foo {
    fn inc(&mut self) {
        self.a += 1;
    }
}
