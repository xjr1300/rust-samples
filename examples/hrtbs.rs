/// HRTBs: Higher-Rank Trait Bounds: 高階トレイト境界

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

// impl<F> Closure<F>
// where
//     F: Fn(&(u8, u16)) -> &u8,
// {
//     fn call(&self) -> &u8 {
//         (self.func)(&self.data)
//     }
// }
//
// fn do_it(data: &(u8, u16)) -> &u8 {
//     &data.0
// }
//
// fn main() {
//     let clo = Closure {
//         data: (0, 1),
//         func: do_it,
//     };
//     println!("{}", clo.call());
// }

// impl<F> Closure<F> {
//     fn call<'a>(&'a self) -> &'a u8 {
//         // compile error: expected function, found `F`
//         // `F`が関数に束縛されていないため、関数として呼び出せない
//         (self.func)(&self.data)
//     }
// }
//
// fn do_it<'b>(data: &'b (u8, u16)) -> &'b u8 {
//     // compile error: borrow expressions cannot be annotated with lifetimes
//     // ライフタイムを指定した借用式を記述できない
//     &'b data.0
// }

impl<F> Closure<F>
where
    for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call<'a>(&'a self) -> &'a u8 {
        (self.func)(&self.data)
    }
}

fn do_it<'b>(data: &'b (u8, u16)) -> &'b u8 {
    &data.0
}

fn main() {
    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    println!("{}", clo.call());
}
