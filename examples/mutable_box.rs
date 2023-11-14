/// mutで作成したBoxの場合、Boxがラップしている値を変更可能!!!（しらなんだ）
#[derive(Debug, Clone, /*Copy,*/ PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, /*Copy,*/ PartialEq, Eq, Hash, Default)]
struct BoxedPoint {
    point: Box<Point>,
}

impl BoxedPoint {
    fn new(point: Point) -> Self {
        Self {
            point: Box::new(point),
        }
    }

    fn mutable_borrow(&mut self) -> &mut Box<Point> {
        &mut self.point
    }
}

fn main() {
    let point = Point::default();

    let mut mutable_boxed_point = Box::new(point);
    mutable_boxed_point.x = 10;
    mutable_boxed_point.y = 5;
    assert_eq!(10, mutable_boxed_point.x);
    assert_eq!(5, mutable_boxed_point.y);

    let point = Point::default();
    let mut boxed_point = BoxedPoint::new(point);
    boxed_point.mutable_borrow().x = 10;
    boxed_point.mutable_borrow().y = 5;
    assert_eq!(10, boxed_point.point.x);
    assert_eq!(5, boxed_point.point.y);

    let other_point = Box::new(Point::default());
    *boxed_point.mutable_borrow() = other_point;
    assert_eq!(0, boxed_point.point.x);
    assert_eq!(0, boxed_point.point.y);
}
