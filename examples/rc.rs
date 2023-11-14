use std::cell::RefCell;
use std::rc::Rc;

struct Person {
    name: String,
    age: u16,
}

fn main() {
    let person = Rc::new(RefCell::new(Person {
        name: String::from("Taro"),
        age: 20,
    }));
    let cloned_person = Rc::clone(&person);
    // 内部可変性を利用して、値を変更
    person.borrow_mut().name = String::from("Jiro");
    cloned_person.borrow_mut().age += 1;

    assert_eq!("Jiro", cloned_person.borrow().name);
    assert_eq!(21, person.borrow().age);
}
