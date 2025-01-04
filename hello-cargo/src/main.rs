struct Student {
    name: String,
    level: u8,
    is_bool: bool,
}
struct Grades(char, char, char, char, f32);
#[derive(Debug)]
struct KeyPress(String, char);
#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}
#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}
#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}
#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}
fn main() {
    println!("Hello, world!");
    println!("First alphabet is {} and Next alphabet is {} ", "A", 'B');
    let mut a_number: i32 = 100;
    println!("number is {}", a_number);
    a_number = 10;
    println!("number is {}", a_number);

    let shadow_num: i32 = 5;
    println!("shadow number is {}", shadow_num);
    let shadow_num: i32 = 5 + shadow_num;
    println!("shadow number is {}", shadow_num);
    let shadow_num: i32 = shadow_num * 2;
    println!("shadow number is {}", shadow_num);

    let is_boolean: bool = 1 < 4;
    println!("1 < 4 is {}", is_boolean);

    let char_1: char = 'H';
    let char_2: char = 'W';
    let string_1: &str = "ello ";
    let string_2: &str = "orld";
    println!("{}{}{}{}!", char_1, string_1, char_2, string_2);

    let tuple = ('E', 5i32, true);
    println!("{}{}{}", tuple.0, tuple.1, tuple.2);

    let user = Student {
        name: String::from("User1"),
        is_bool: true,
        level: 2,
    };
    let mark = Grades('A', 'B', 'C', 'D', 3.22);
    println!(
        "{}{}{}{}{}{}{}{}",
        user.name, user.level, user.is_bool, mark.0, mark.1, mark.2, mark.3, mark.4
    );

    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click is {}, {}", click.x, click.y);
    let keys = KeyPress(String::from("ctrl"), 'N');
    println!("Keys pressed: {}, {}", keys.0, keys.1);

    let we_load = WebEvent::WELoad(true);
    let we_click = WebEvent::WEClick(click);
    let we_key = WebEvent::WEKeys(keys);
    println!(
        "WebEvent enum structure: {:#?}, {:#?}, {:#?}",
        we_load, we_click, we_key
    );
    goodbye("Goodbye");
    vec();

    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();
    let mut order = 1;
    let mut car: Car;
    car = car_factory(order, 1000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    order = order + 1;
    car = car_factory(order, 2000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
    order = order + 1;
    car = car_factory(order, 3000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
    order = order + 1;
    car = car_factory(order, 4000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    hash_map();
}

fn goodbye(text: &str) {
    println!("text: {}", text)
}

fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut color = order as usize;
    if color > 4 {
        color = color - 4;
    }

    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }
    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn vec() {
    let three_nums = vec![153, 58, 00];
    println!("Init vector: {:?}", three_nums);
    let zeroes = vec![0; 5];
    println!("Zero: {:?}", zeroes);

    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);
    fruit.pop();
    println!("Fruits: {:?}", fruit)
}

fn car_quality(miles: u32) -> (Age, u32) {
    let quality = (Age::New, miles);
    quality;
    if miles > 0 {
        return (Age::Used, miles);
    }
    (Age::New, miles)
}

fn hash_map() {
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurare."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great exaples."),
    );
    let book: &str = "Programming in Rust";
    println!("Reaview for '{}': '{:?}'", book, reviews.get(book));

    let obsolete: &str = "Ancient Roman History";
    println!("'{}' removed.", obsolete);
    reviews.remove(obsolete);
    println!("Review for '{}': '{:?}'", obsolete, reviews.get(obsolete))
}
