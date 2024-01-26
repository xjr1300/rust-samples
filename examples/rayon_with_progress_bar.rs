use std::thread::sleep;
use std::time::Duration;

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let v: Vec<_> = (0..100000).collect();
    let v2: Vec<_> = v
        .par_iter()
        .progress_count(v.len() as u64)
        .map(|i| {
            sleep(Duration::from_millis(50));
            i + 1
        })
        .collect();
    assert_eq!(v2[0], 1);
}
