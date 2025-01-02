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

    // let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    // println!(
    //     "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
    //     car.color, car.transmission, car.convertible, car.mileage
    // );
    // car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    // println!(
    //     "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
    //     car.color, car.transmission, car.convertible, car.mileage
    // );
    // car = car_factory(String::from("Yellow"), Transmission::SemiAuto, true);
    // println!(
    //     "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
    //     car.color, car.transmission, car.convertible, car.mileage
    // );
    vec();

    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut car: Car;
    let mut engine = Transmission::Manual;

    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}

fn goodbye(text: &str) {
    println!("text: {}", text)
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color: color,
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
    quality
}
