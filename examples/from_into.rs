#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<Number> for i32 {
    fn from(item: Number) -> Self {
        item.value
    }
}

fn main() {
    let num = Number::from(30);
    assert_eq!(30, num.value);
    let num2: Number = 40.into();
    assert_eq!(40, num2.value);

    let num_value: i32 = num.into();
    assert_eq!(30, num_value);
}
