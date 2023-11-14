// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output {
//         if self >= 0 {      // 次で修正するエラーの発生箇所
//             self as <Self as IAbs>::Output
//         } else {
//             -self as <Self as IAbs>::Output
//         }
//     }
// }

// /// binary operation `>=` cannot be applied to type `Self`
// /// を修正するために、PartialEqとPartialOrdをトレイト境界に追加
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output // 次で修正するエラーの発生箇所
//     where
//         Self: PartialEq + PartialOrd,
//     {
//         if self >= 0 {
//             self as <Self as IAbs>::Output
//         } else {
//             -self as <Self as IAbs>::Output
//         }
//     }
// }

// /// the size for values of type `Self` cannot be known at compilation time
// /// doesn't have a size known at compile-time
// /// を修正するために、Sizedをトレイト境界に追加
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output
//     where
//         Self: PartialEq + PartialOrd + Sized,
//     {
//         if self >= 0 {      // 次で修正するエラーの発生箇所
//             self as <Self as IAbs>::Output
//         } else {
//             -self as <Self as IAbs>::Output
//         }
//     }
// }

// /// mismatched types
// ///     expected type parameter `Self`
// ///     found type `{integer}`
// /// `IAbs`トレイトは、i8, i16, i32, i64が実装
// /// 比較演算子で使用している0をi8にして、i16, i32, i64が実装するFrom<u8>を利用
// /// できるため、into()する。
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output
//     where
//         Self: PartialEq + PartialOrd + Sized,
//     {
//         if self >= 0_u8.into() {
//             self as <Self as IAbs>::Output
//         } else {
//             -self as <Self as IAbs>::Output  // 次で修正するエラーの発生箇所
//         }
//     }
// }

use std::ops::Neg;

// /// cannot apply unary operator `-` to type `Self`
// /// を修正するために、Negをトレイト境界に追加
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output
//     where
//         Self: PartialEq + PartialOrd + Neg + Sized,
//     {
//         if self >= 0_u8.into() {    // 次で修正するエラーの発生箇所
//             self as <Self as IAbs>::Output
//         } else {
//             -self as <Self as IAbs>::Output
//         }
//     }
// }

// /// the trait bound `Self: From<u8>` is not satisfied
// /// required for `u8` to implement `Into<Self>`
// ///
// /// consider further restricting `Self`: `, Self: From<i8>`
// /// を修正するために、`Self: From<i8>`をトレイト境界に追加
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output
//     where
//         Self: PartialEq + PartialOrd + Neg + Sized + From<i8>,
//     {
//         if self >= 0_i8.into() {
//             self as <Self as IAbs>::Output  // non-primitive cast: `Self` as `<Self as IAbs>::Output`
//         } else {
//             -self as <Self as IAbs>::Output
//         }
//     }
// }

// /// non-primitive cast: `Self` as `<Self as IAbs>::Output`
// /// an `as` expression can only be used to convert between
// /// primitive types or to coerce to a specific trait object
// /// を修正
// ///
// /// `as`式は、プリミティブ型間の変換、または特定のトレイトオブジェクトに強制変換するためにのみ使用
// ///
// /// Selfを<Self as IAbs>::Outputに変換するため、TryIntoでSelfを<Self as IAbs>::Outputに
// /// 変換できる制約を追加
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output
//     where
//         Self: PartialEq + PartialOrd + Neg + Sized + From<i8> + TryInto<<Self as IAbs>::Output>,
//     {
//         if self >= 0_i8.into() {
//             self.try_into()  // 次に修正するエラーの発生箇所
//         } else {
//             (-self).try_into()
//         }
//     }
// }

// /// mismatched types
// /// expected associated type `<Self as IAbs>::Output`
// ///        found enum `Result<<Self as IAbs>::Output, <Self as TryInto<<Self as IAbs>::Output>>::Error>`
// /// を修正
// ///
// /// try_into()は、Resultを変換する。
// /// 例えば。正のi32は、u32に常に変換可能であるため、unwrap()でResultのラップを剥がす。
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output
//     where
//         Self: PartialEq + PartialOrd + Neg + Sized + From<i8> + TryInto<<Self as IAbs>::Output>,
//     {
//         if self >= 0_i8.into() {
//             self.try_into().unwrap()    // 次に修正するエラーの発生場所
//         } else {
//             (-self).try_into().unwrap()
//         }
//     }
// }

use std::fmt::Debug;

// /// `<Self as TryInto<<Self as IAbs>::Output>>::Error` doesn't implement `Debug`
// /// the trait `Debug` is not implemented for `<Self as TryInto<<Self as IAbs>::Output>>::Error`
// /// を修正
// ///
// /// <Self as TryInto<<Self as IAbs>::Output>>::Errorが、Debugを実装していないことを示しているため、
// /// <Self as TryInto<<Self as IAbs>::Output>>::Error: Debugをトレイト境界に追加
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output
//     where
//         Self: PartialEq + PartialOrd + Neg + Sized + From<i8> + TryInto<<Self as IAbs>::Output>,
//         <Self as TryInto<<Self as IAbs>::Output>>::Error: Debug,
//     {
//         if self >= 0_i8.into() {
//             self.try_into().unwrap()
//         } else {
//             (-self).try_into().unwrap() // 次に修正するエラーの発生場所
//         }
//     }
// }

// /// the trait bound `<Self as IAbs>::Output: From<<Self as Neg>::Output>` is not satisfied
// /// required for `<Self as Neg>::Output` to implement `Into<<Self as IAbs>::Output>`
// /// required for `<Self as IAbs>::Output` to implement `TryFrom<<Self as Neg>::Output>`
// /// required for `<Self as Neg>::Output` to implement `TryInto<<Self as IAbs>::Output>`
// /// を修正
// ///
// /// コンパイラが、<Self as IAbs>::Output: From<<Self as Neg>::Output>の制約を提案しているが、例えば
// /// From<i8> -> u8は実装されていない。
// /// TryFrom<i8> for u8は実装されているため、<Self as IAbs>::Output: TryFrom<<Self as Neg>::Output>を追加
// ///
// /// 数値の絶対値を求めるトレイト
// trait IAbs {
//     type Output;
//
//     fn iabs(self) -> <Self as IAbs>::Output
//     where
//         Self: PartialEq + PartialOrd + Neg + Sized + From<i8> + TryInto<<Self as IAbs>::Output>,
//         <Self as TryInto<<Self as IAbs>::Output>>::Error: Debug,
//         <Self as IAbs>::Output: TryFrom<<Self as Neg>::Output>,
//     {
//         if self >= 0_i8.into() {
//             self.try_into().unwrap()
//         } else {
//             (-self).try_into().unwrap()     // 次に修正するエラーが発生した箇所
//         }
//     }
// }

/// `<<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Error` doesn't implement `Debug`
/// the trait `Debug` is not implemented for `<<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Error`
/// を修正
///
/// <<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Errorが、Debugトレイトを実装していないと示しているため、
/// <<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Error: Debugを制約に追加
///
/// 数値の絶対値を求めるトレイト
trait IAbs {
    type Output;

    fn iabs(self) -> <Self as IAbs>::Output
    where
        Self: PartialEq + PartialOrd + Neg + Sized + From<i8> + TryInto<<Self as IAbs>::Output>,
        <Self as TryInto<<Self as IAbs>::Output>>::Error: Debug,
        <Self as IAbs>::Output: TryFrom<<Self as Neg>::Output>,
        <<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Error: Debug,
    {
        if self >= 0_i8.into() {
            self.try_into().unwrap()
        } else {
            (-self).try_into().unwrap()
        }
    }
}

impl IAbs for i8 {
    type Output = u8;
}

impl IAbs for i16 {
    type Output = u16;
}

impl IAbs for i32 {
    type Output = u32;
}

impl IAbs for i64 {
    type Output = u64;
}

fn main() {
    assert_eq!(0_u8, 0_i8.iabs());
    assert_eq!(1_u8, 1_i8.iabs());
    assert_eq!(1_u8, (-1 as i8).iabs());

    assert_eq!(0_u16, 0_i16.iabs());
    assert_eq!(1_u16, 1_i16.iabs());
    assert_eq!(1_u16, (-1 as i16).iabs());

    assert_eq!(0_u16, 0_i16.iabs());
    assert_eq!(1_u16, 1_i16.iabs());
    assert_eq!(1_u16, (-1 as i16).iabs());

    assert_eq!(0_u32, 0_i32.iabs());
    assert_eq!(1_u32, 1_i32.iabs());
    assert_eq!(1_u32, (-1 as i32).iabs());

    assert_eq!(0_u64, 0_i64.iabs());
    assert_eq!(1_u64, 1_i64.iabs());
    assert_eq!(1_u64, (-1 as i64).iabs());
}
