macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    }
}

fn main() {
    // ::std::io::_print(format_args!("{0}\n", 1));
    println!("{}", find_min!(1));
    // ::std::io::_print(format_args!("{0}\n", std::cmp::min(1 + 2, 2)));
    println!("{}", find_min!(1 + 2, 2));
    // ::std::io::_print(
    //     format_args!("{0}\n", std::cmp::min(5, std::cmp::min(2 * 3, 4))),
    // );
    println!("{}", find_min!(5, 2 * 3, 4));
    // ::std::io::_print(
    //     format_args!(
    //         "{0}\n",
    //         std::cmp::min(
    //             1,
    //             std::cmp::min(
    //                 2,
    //                 std::cmp::min(
    //                     3,
    //                     std::cmp::min(
    //                         4,
    //                         std::cmp::min(5, std::cmp::min(6, std::cmp::min(7, 8))),
    //                     ),
    //                 ),
    //             ),
    //         ),
    //     ),
    // );
    println!("{}", find_min!(1, 2, 3, 4, 5, 6, 7, 8));
}
