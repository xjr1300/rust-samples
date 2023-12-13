enum MyEnum {
    A,            // content: 0byte
    B(i32),       // content: 4byte
    C([i32; 10]), // content: 40byte
}

#[allow(dead_code)]
enum F64Enum {
    A(f64), // content: 8byte
}

#[allow(dead_code)]
enum F64Enum2 {
    A(f64), // content: 8byte
    B(f64), // content: 8byte
}

fn main() {
    println!("MyEnum: {}", std::mem::size_of::<MyEnum>());
    let a = MyEnum::A;
    let b = MyEnum::B(1);
    let c = MyEnum::C([1; 10]);
    println!("MyEnum::A: {}", std::mem::size_of_val(&a));
    println!("MyEnum::B: {}", std::mem::size_of_val(&b));
    println!("MyEnum::C: {}", std::mem::size_of_val(&c));

    println!("F64Enum: {}", std::mem::size_of::<F64Enum>());
    println!("F64Enum2: {}", std::mem::size_of::<F64Enum2>());
}
