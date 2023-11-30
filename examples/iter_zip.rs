fn main() {
    // 要素数が等しい場合
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5, 6];
    let mut iter = s1.iter().zip(s2.iter());
    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);

    // 最初のイテレーターの要素数が多い場合
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5];
    let mut iter = s1.iter().zip(s2.iter());
    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), None);

    // 後のイテレーターの要素数が多い場合
    let s1 = &[1, 2];
    let s2 = &[4, 5, 6];
    let mut iter = s1.iter().zip(s2.iter());
    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), None);

    // 文字列で要素数が等しい場合
    let s1 = "abc\ndef\nghi\n";
    let s2 = "012\n345\n678\n";
    let mut iter = s1.lines().zip(s2.lines());
    assert_eq!(iter.next(), Some(("abc", "012")));
    assert_eq!(iter.next(), Some(("def", "345")));
    assert_eq!(iter.next(), Some(("ghi", "678")));
    assert_eq!(iter.next(), None);

    // 文字列で要素数が異なる場合
    let s1 = "abc\ndef\nghi\n";
    let s2 = "012\n345\n";
    let mut iter = s1.lines().zip(s2.lines());
    assert_eq!(iter.next(), Some(("abc", "012")));
    assert_eq!(iter.next(), Some(("def", "345")));
    assert_eq!(iter.next(), None);
}
