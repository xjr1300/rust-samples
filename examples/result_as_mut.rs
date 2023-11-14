/// ```
/// pub fn as_mut(&mut self) -> Result<&mut T, &mut E>
/// ```
///
/// Converts from &mut Result<T, E> to Result<&mut T, &mut E>.
///
/// `&mut Result<T, E>`から`Result<&mut T, &mut E>`に変換する。
fn main() {
    let mut x: Result<i32, i32> = Ok(2);
    mutate(&mut x);
    assert_eq!(x, Ok(4));

    let mut y: Result<i32, i32> = Err(3);
    mutate(&mut y);
    assert_eq!(y, Err(9));
}

fn mutate(x: &mut Result<i32, i32>) {
    match x.as_mut() {
        Ok(v) => *v *= 2,
        Err(e) => *e *= 3,
    };
}
