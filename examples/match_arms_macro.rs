macro_rules! match_arms {
    ($target:expr, $($matcher:pat => $result:expr),*) => {
        match $target {
            $($matcher => $result),*
        }
    }
}

fn option_u8_fn(target: u8) -> Option<u8> {
    match_arms! {
        target,
        0 => None,
        1 => {
            let _ = 1;

            Some(1)
        },
        _ => unreachable!()
    }
}

macro_rules! match_arms2 {
    ($($matcher:pat => $result:expr),*) => {
        match 1 {
            $($matcher => $result),*
        }
    }
}

fn main() {
    assert_eq!(Some(1_u8), option_u8_fn(1));

    let result = match_arms2! {
        0 => None,
        1 => Some(1_u8),
        _ => unreachable!()
    };
    assert_eq!(Some(1_u8), result);
}
