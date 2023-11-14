/// 型パラメータX, Yを受け取るPoint構造体
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<X, Y> {
    x: X,
    y: Y,
}

fn main() {
    // point1の型は、Point<i32, f64>
    let point1 = Point { x: 1, y: 2.0 };
    println!("{:?}", point1);

    // point2の型は、Point<f64, i32>
    let point2 = Point { x: 1.0, y: 2 };
    println!("{:?}", point2);

    // 次は、コンパイラは、型が異なることをエラーとして通知
    // point1とpoint2は型が異なるため、比較できない
    // assert!(point1 == point2);

    // point3の型は、point1と同じ
    // よって、PartialEqトレイトの自動実装によって比較可能
    let point3 = Point { x: 1, y: 2.0 };
    assert!(point1 == point3);
}
