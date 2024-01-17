//! イテレーターを取得した後にベクタを変更するとエラーになる例
#[allow(unused_mut)]

fn main() {
    #[allow(clippy::useless_vec)]
    let mut v = vec![1, 2, 3]; // warning: unused_mut
    let it = v.iter();
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    //  --> examples/get_vec_iter_then_push_item.rs:4:5
    //   |
    // 3 |     let it = v.iter();
    //   |              -------- immutable borrow occurs here
    // 4 |     v.push(4);
    //   |     ^^^^^^^^^ mutable borrow occurs here
    // 5 |     println!("{:?}", it);
    //   |                      -- immutable borrow later used here
    // v.push(4);
    println!("{:?}", it);
}
