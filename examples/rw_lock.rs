use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);

    // 多くの読み込みロックは同時に取得できる
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("r1 = {}, r2 = {}", r1, r2);
    } // r1 と r2 はここでスコープを抜ける

    // 一つの書き込みロックしか取得できない
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("w = {}", w);
    } // w はここでスコープを抜ける
}
