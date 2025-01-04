use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
struct Student {
    name: String,
    level: u8,
    is_bool: bool,
}
struct Person {
    first: String,
    middle: Option<String>,
    last: String,
}
#[derive(Debug)]
struct DivisionByZeroError;
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

    let mut orders: HashMap<i32, Car> = HashMap::new();
    let mut order = 1;
    let mut car: Car;
    car = car_factory(order, 1000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    let mut miles = 0;
    for order in 1..12 {
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }

    hash_map();
    loop_fn();
    error_fn();

    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");

    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));

    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}

fn goodbye(text: &str) {
    println!("text: {}", text)
}

fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut color = order as usize;
    while color > 4 {
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

fn loop_fn() {
    let mut conter = 1;
    let stop_loop = loop {
        conter *= 2;
        if conter > 100 {
            break conter;
        }
    };
    println!("Break the loop = {}.", stop_loop);
    conter = 0;
    while conter < 5 {
        println!("loop count: {}", conter);
        conter += 1;
    }
    let birds = ["ostrich", "peacock", "stork"];
    for bird in birds.iter() {
        println!("{} is Bird.", bird);
    }
    for number in 0..5 {
        println!("{}", number)
    }
}

fn error_fn() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 1, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("This is coconut"),
            Some(name) => println!("Fruits name is '{}'", name),
            None => println!("This fruits is None"),
        }
    }

    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {}
    }
    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }
}
fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    if let Some(middle) = &person.middle {
        full_name.push_str(&middle);
        full_name.push_str(" ")
    }

    full_name.push_str(&person.last);
    full_name
}

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}
fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };
    Ok(string)
}
