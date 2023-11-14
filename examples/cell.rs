use std::cell::Cell;

struct Foo {
    x: Cell<i32>,
}

fn main() {
    // 可変変数で束縛していない
    let c = Cell::new(1);
    // Cellが格納している値を変更
    c.set(2);
    assert_eq!(2, c.get());

    let f = Foo { x: Cell::new(1) };
    f.x.set(f.x.get() + 1);
    assert_eq!(2, f.x.get());
}
