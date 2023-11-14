use std::cell::RefCell;

/// Boxでサイズを計算できない型をヒープに確保することで、サイズを計算できる型を定義
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn print(&self) {
        let mut current = self;
        loop {
            match current {
                Cons(value, next) => {
                    print!("{} -> ", value);
                    // 自動で参照外し
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

trait Draw {
    fn draw(&self);
}

/// Drawはトレイトのため、コンパイル時にサイズを計算できないためBoxでラップ
/// Vec<Box<dyn Draw>>はサイズを計算可能
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

struct Point;

impl Draw for Point {
    fn draw(&self) {
        println!("Point");
    }
}

struct Rectangle;

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Rectangle");
    }
}

struct Circle;

impl Draw for Circle {
    fn draw(&self) {
        println!("Circle");
    }
}

struct Person {
    name: String,
    age: u16,
}

fn main() {
    {
        // ヒープにi32型の値を確保
        let b = Box::new(5);
        // Boxはポインタであるため参照外しが必要
        println!("{}", *b);
        // しかし、BoxはDerefを実装しているため、自動的に参照外し
        println!("{}", b);
        // スコープを抜けると、Boxがラップしている値とボックス自体が解放
    }
    {
        // Consリスト
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        list.print();
        // 以下は、list3でエラーが発生する。
        // let list2 = Cons(4, Box::new(list)); // listは所有権を失う
        // let list3 = Cons(4, Box::new(list)); // listにリストが束縛されていないため、listを使えない
    }
    {
        // コンポーネントはトレイトオブジェクトであるため、Vecに異なる型を格納可能
        let screen = Screen {
            components: vec![Box::new(Point), Box::new(Rectangle), Box::new(Circle)],
        };
        for component in screen.components.iter() {
            component.draw();
        }
    }
    {
        let person = Box::new(RefCell::new(Person {
            name: String::from("Taro"),
            age: 20,
        }));
        person.borrow_mut().name = String::from("Jiro");
        person.borrow_mut().age += 1;
        assert_eq!("Jiro", person.borrow().name);
        assert_eq!(21, person.borrow().age);
    }
}
