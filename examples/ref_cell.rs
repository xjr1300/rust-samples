use std::cell::RefCell;

fn main() {
    let c = RefCell::new(String::from("hello"));
    c.borrow_mut().push_str(" world");
    println!("{}", c.borrow());
}
