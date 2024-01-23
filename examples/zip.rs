fn main() {
    let v1 = [0, 1, 2, 3, 4, 5];
    let v2 = [5, 4, 3, 2, 1, 0, -1, -2];

    // zipが生成するイテレーターは、2つのイテレーターのうち、イテレーターが短い方の長さに合わせられる
    println!(
        "v1.iter().zip(v2.iter()), v1.len() = {}, v2.len() = {}",
        v1.len(),
        v2.len()
    );
    for (i, j) in v1.iter().zip(v2.iter()) {
        println!("{} {}", i, j);
    }
    println!();

    println!(
        "v2.iter().zip(v1.iter()), v1.len() = {}, v2.len() = {}",
        v1.len(),
        v2.len()
    );
    for (i, j) in v2.iter().zip(v1.iter()) {
        println!("{} {}", i, j);
    }
}
