fn main() {
    println!("Hello, world!");
    println!("First alphabet is {} and Next alphabet is {} ", "A", 'B');
    let mut a_number: i32 = 100;
    println!("number is {}", a_number);
    a_number = 10;
    println!("number is {}", a_number);

    let shadow_num:i32 = 5;
    println!("shadow number is {}", shadow_num);
    let shadow_num: i32 = 5 + shadow_num;
    println!("shadow number is {}", shadow_num);
    let shadow_num:i32= shadow_num * 2;
    println!("shadow number is {}", shadow_num);

    let is_boolean: bool = 1<4;
    println!("1 < 4 is {}", is_boolean);

    let char_1: char = 'H';
    let char_2: char = 'W';
    let string_1: &str = "ello ";
    let string_2: &str = "orld";
    println!("{}{}{}{}!", char_1, string_1, char_2,string_2);
}
