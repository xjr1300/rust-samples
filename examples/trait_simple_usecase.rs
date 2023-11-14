use std::fmt::Debug;

/// Animalトレイトを定義
///
/// Animalトレイトを実装する構造体は、Debugトレイトの実装を強制
trait Animal: Debug {
    /// デフォルト実装を持たないメソッド
    fn name(&self) -> &str;

    /// デフォルト実装を持つメソッド
    fn talk(&self) {
        println!("I'm {}.", self.name());
    }

    /// デフォルト実装を持たないメソッド
    fn set_name(&mut self, name: &str);
}

/// 犬
///
/// `#[derive(Debug)]`をコメントアウトすると、Debugトレイトの実装が必要なことを示すコンパイルエラーが発生
#[derive(Debug)]
struct Dog {
    name: String,
}

impl Dog {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Animalトレイトを実装
impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

/// Debugトレイトを自動導出しないで、独自に実装
struct Cat {
    name: String,
}

impl Cat {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

/// Debugトレイトを実装
impl Debug for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cat").field("name", &self.name).finish()
    }
}

fn main() {
    let dog = Dog::new("Pochi");
    assert_eq!("Pochi", dog.name);
    dog.talk();

    let cat = Cat::new("Tama");
    assert_eq!("Tama", cat.name);
    cat.talk();
    println!();

    // 次はコンパイルできない
    // animalsは、Vec<Dog>型であるため、Cat型の値を格納できない
    // let animals = vec![dog, cat];

    // 次はコンパイルできない
    // animals変数の型のサイズを決定する必要があるが、Animalトレイトでは、コンパイル時にVecの要素のサイズを決定できない
    // コンパイラは、Vecの要素の型を`dyn Animal`に変更することを提案
    // let animals: Vec<Animal> = vec![dog, cat];

    // 次はコンパイルできない
    // 上記同様で、コンパイル時にサイズを決定できない
    // let animals: Vec<dyn Animal> = vec![dog, cat];

    println!("--- The Vec that stores the Animal trait object wrapped in each Box ---");
    // BoxでラップしたAnimalトレイトを実装した構造体の値を格納することで、Vecの要素のサイズがusizeに決定
    // Boxでラップすることで、dog及びcat変数は、所有型を失う
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
    for animal in animals {
        animal.talk();
    }
    println!();

    let dog = Dog::new("Pochi");
    let cat = Cat::new("Tama");

    println!("--- The Vec That stores the reference of Animal trait objects ---");
    // Animalトレイトオブジェクトの参照を格納することで、Vecの要素のサイズがusizeに決定
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];
    for animal in animals {
        animal.talk();
    }
    // 参照であるため、当然doc及びcat変数は所有権を保有
    dog.talk();
    cat.talk();
    println!();

    println!("--- The trait objects can debug print ---");
    print_debug(&dog);
    print_debug(&cat);
    println!();

    println!("--- Change the dog name that the dog is wrapped in Box included Vec ---");
    let mut animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
    animals[0].set_name("Taro");
    print_debug(animals[0].as_ref());
}

// 次の関数はコンパイルできない
// 引数animalの型がAnimalトレイトであるため、引数のサイズを決定できない
// コンパイラは、引数animalの型を`dyn Animal`に変更するように提案
// fn print_debug(animal: Animal) {
//     println!("{:?}", animal);
// }

// 次の関数はコンパイルできない
// トレイトオブジェクトは、`dyn`キーワードを含めなくてはならないことを示すコンパイルエラーが発生
// コンパイラは、引数animalの型を`&dyn Animal`に変更するように提案
// fn print_debug(animal: &Animal) {
//     println!("{:?}", animal);
// }

// 引数animalの型を&dyn Animalに変更することで、引数animalのサイズはusizeに決定
fn print_debug(animal: &dyn Animal) {
    println!("{:?}", animal);
}
