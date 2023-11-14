use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn print(&self) {
        let mut current = self;
        loop {
            match current {
                Cons(value, next) => {
                    print!("{} -> ", value);
                    current = next;
                }
                Nil => {
                    println!("Nil");
                    break;
                }
            }
        }
    }
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    // bとcでaを共有
    let b = Cons(100, Rc::clone(&a));
    let c = Cons(101, Rc::clone(&a));
    // aの参照カウンタは3
    assert_eq!(3, Rc::strong_count(&a));
    // 出力
    a.print();
    b.print();
    c.print();
}
