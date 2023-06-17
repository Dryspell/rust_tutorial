#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, u8};

fn example1_io() {
    println!("What's your name?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let greeting = "Nice to meet you!";

    println!("Hello, {}! {}", name.trim(), greeting);
}

fn example2_io() {
    const ONE_MILLION: u32 = 1_000_000;
    const PI: f32 = std::f32::consts::PI;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Please type a number!");
    age += 1;
    print!("age: {}, and I want ${}", age, ONE_MILLION);
}

fn example3_std_types() {
    println!("Max 8: {}", std::u8::MAX);
    println!("Max u16: {}", std::u16::MAX);
    println!("Max u32: {}", std::u32::MAX);
    println!("Max u64: {}", std::u64::MAX);
    println!("Max u128: {}", std::u128::MAX);
    // println!("Max usize: {}", std::usize::MAX);

    println!("Max i8: {}", std::i8::MAX);
    println!("Max i16: {}", std::i16::MAX);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);

    let is_true = true;

    // characters are denoted with single quotes
    let my_grade = 'A';

    // tuples are denoted with parentheses
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // arrays are denoted with square brackets
    let arr = [1, 2, 3, 4, 5];

    // arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&arr));

    // get slice of array
    let slice: &[i32] = &arr[1..3];

    // print array
    println!("{:?}", arr);

    // print tuple
    println!("{:?}", tup);

    // print slice
    println!("{:?}", slice);

    // print boolean
    println!("{}", is_true);

    // print character
    println!("{}", my_grade);

    // note the excessive floating point precision
    let num_1: f32 = 1.111_111_111_111_11;
    let num_2: f32 = 1.11111111111111;

    println!("f32: {}", (num_1 + num_2));

    // basic math
    println!("5 + 4 = {}", 5 + 4);

    // order of operations
    println!("5 + 4 * 3 = {}", 5 + 4 * 3);

    let random_num = rand::thread_rng().gen_range(1..11);

    println!("Random Number: {}", random_num);
}

fn example4_control_flow() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);

    // Match
    let country_code = 46;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid",
    };

    println!("Country: {}", country);

    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("{} is too young to vote", my_age),
        Ordering::Greater => println!("Hey you're {}, you better vote!", my_age),
        Ordering::Equal => println!("Congratulations on turning {}! Please vote!", my_age),
    }
}

fn example5_arrays_and_loops() {
    let arr_1: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("arr_1 has {} elements", arr_1.len());

    let mut loop_index = 0;
    loop {
        println!("loop_index: {}, value: {}", loop_index, arr_1[loop_index]);

        if arr_1[loop_index] == 7 {
            break;
        }
        if arr_1[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        loop_index += 1;
    }

    loop_index = 0;
    while loop_index < arr_1.len() {
        println!(
            "while loop_index: {}, value: {}",
            loop_index, arr_1[loop_index]
        );
        loop_index += 1;
    }

    for val in arr_1.iter() {
        println!("for loop value: {}", val);
    }

    for x in 0..10 {
        println!("x: {}", x);
    }
}

fn example6_tuples() {
    let rand_tuple: (String, u8, f64) = ("Derek".to_string(), 40, 50_000.0);

    let rand_tuple_2: (&str, i8) = ("Derek", 40);

    println!("Name: {}", rand_tuple_2.0);

    // Destructuring
    let (v1, v2, v3) = rand_tuple;

    println!("Name: {}", v1);
}

fn example7_strings() {
    let mut st1 = String::new();
    st1.push('H');
    st1.push_str("ello world");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("l", "x");

    let st3 = String::from(" xlskd hsfsd qhrwel dks");

    println!("{}{}{}", st2, st3, st1);

    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "random string";
    let mut st5: String = st4.to_string();

    println!("{}", st5.len());

    st5.push_str(" some more stuff");

    let byte_arr1 = st5.as_bytes();

    let st6 = &st5[0..6];
    println!("st6: {}", st6.len());
    st5.clear();

    let st6 = String::from("Hello world");
    let st7 = String::from("world");

    let st8 = st6 + &st7;
    println!("{}", st8);

    // Notice the borrow checker here does not let you do this
    // println!("{}", str6);
    // println!("{}", str7);

    for char in st8.bytes() {
        println!("{}", char);
    }
}

fn example8_casting() {
    let a: i32 = 1;
    let b: i64 = 2;
    let c: i32 = b as i32;

    println!("{}", c);

    let c2: i32 = 1000;
    let d: i8 = c2 as i8;

    println!("{}", d);

    let e: f32 = 2345.345;
    let f: u8 = e as u8;

    println!("{}", f);

    let g: u8 = 123;
    let h: i8 = g as i8;
}

fn example9_enums() {
    impl Movement {
        fn move_avatar(&self) {
            match self {
                Movement::Up => println!("Avatar moving up"),
                Movement::Down => println!("Avatar moving down"),
                Movement::Left => println!("Avatar moving left"),
                Movement::Right => println!("Avatar moving right"),
            }
        }
    }

    enum Movement {
        Up,
        Down,
        Left,
        Right,
    }

    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    avatar1.move_avatar();
    avatar2.move_avatar();
    avatar3.move_avatar();
    avatar4.move_avatar();

    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekday(&self) -> bool {
            !matches!(self, &Day::Saturday | &Day::Sunday)
            // Equivalent to:
            // match self {
            //     &Day::Saturday | &Day::Sunday => false,
            //     _ => true,
            // }
            //
        }
    }

    let day1 = Day::Monday;
    let day2 = Day::Saturday;

    println!("Is day1 a weekday: {}", day1.is_weekday());
    println!("Is day2 a weekday: {}", day2.is_weekday());

    let today = Day::Friday;

    match today {
        Day::Monday => println!("It's Monday"),
        Day::Tuesday => println!("It's Tuesday"),
        Day::Wednesday => println!("It's Wednesday"),
        Day::Thursday => println!("It's Thursday"),
        Day::Friday => println!("It's Friday"),
        Day::Saturday => println!("It's Saturday"),
        Day::Sunday => println!("It's Sunday"),
    }
}

fn example10_vectors() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("{:?}", vec2);
    println!("{}", vec2[2]);

    let second: &i32 = &vec2[1];

    match vec2.get(2) {
        Some(x) => println!("Item 2 is {}", x),
        None => println!("Sorry, this vector is too short."),
    }

    for i in &mut vec2 {
        *i = i.pow(2);
        println!("{}, {}", i, *i);
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vec length: {}", vec2.len());
    println!("Last element, pop {}", vec2.pop().unwrap());
}

fn example11_functions() {
    fn say_hello() {
        println!("Hello");
    }

    say_hello();

    fn get_sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }

    println!("5 + 4 = {}", get_sum(5, 4));

    let sum_nums = |num1: i32, num2: i32| num1 + num2;
    println!("7 + 8 = {}", sum_nums(7, 8));

    let num_ten = 10;
    let add_10 = |x: i32| x + num_ten;

    println!("5 + 10 = {}", add_10(5));

    // Note the unneeded return keyword
    fn get_sum2(num1: i32, num2: i32) -> i32 {
        return num1 + num2;
    }

    fn sum_list(list: &[i32]) -> i32 {
        let mut sum = 0;
        for i in list {
            sum += i;
        }
        sum
    }

    let list1 = [1, 2, 3, 4, 5];
    println!("Sum of list: {}", sum_list(&list1));
}

fn example12_generics() {
    use std::marker::Copy;
    use std::ops::Add;

    fn get_sum<T: Add<Output = T>>(num1: T, num2: T) -> T {
        num1 + num2
    }
    println!("5 + 4 = {}", get_sum(5, 4));

    // fn get_sum_list<T: Add<Output = T>>(list: &[T]) -> T {
    //     let mut sum = list[0];
    //     for i in list {
    //         sum = sum + *i;
    //     }
    //     sum
    // }
}

fn example13_ownership() {
    // Notice the borrow checker here does not let you do this
    // let str1 = String::from("World");
    // let str2 = str1;
    // println!("Hello {}", str1);

    fn subexample1() {
        let str1 = String::from("World");
        let str2 = str1;
        println!("Hello {}", str2);
    }
    subexample1();

    let str1 = String::from("World");
    let str2 = str1.clone();
    println!("Hello {}", str2);

    fn print_return_str(x: String) -> String {
        println!("{}", x);
        x
    }

    fn change_string(name: &mut String) {
        name.push_str("!!");
    }

    println!("Hello {}", str2);

    // Note borrow checker doesn't let this happen either
    // fn print_str(x: String) {
    //     println!("{}", x);
    // }

    // print_str(str2);
    // println!("Hello {}", str2);
    //

    let mut str3 = String::from("World");
    change_string(&mut str3);

    println!("Hello {}", str3);
}

fn example14_hashmaps() {
    let mut heroes = HashMap::new();

    heroes.insert("Spiderman", "Peter Parker");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Ironman", "Tony Stark");

    for (k, v) in heroes.iter() {
        println!("{}: {}", k, v);
    }

    println!("Spiderman: {}", heroes.get("Spiderman").unwrap());

    let spiderman = "Spiderman";
    if heroes.contains_key(spiderman) {
        println!("Contains key {}", spiderman);

        let the_spiderman = heroes.get(spiderman).unwrap();

        match the_spiderman {
            &"Peter Parker" => println!("Good guy {}!", the_spiderman),
            _ => println!("Bad guy"),
        }
    }
}

fn example15_structs() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob"),
        address: String::from("123 Main"),
        balance: 32.50,
    };

    bob.address = String::from("123 Main St");

    trait Shape {
        fn new(width: f32, height: Option<f32>) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        width: f32,
        height: f32,
    }

    impl Shape for Rectangle {
        fn new(width: f32, height: Option<f32>) -> Self {
            Rectangle {
                width,
                height: height.unwrap_or(width),
            }
        }
        fn area(&self) -> f32 {
            self.width * self.height
        }
    }

    let rec = Rectangle {
        width: 10.0,
        height: 20.5,
    };

    struct Circle {
        radius: f32,
    }

    impl Shape for Circle {
        fn new(width: f32, height: Option<f32>) -> Self {
            Circle { radius: width }
        }
        fn area(&self) -> f32 {
            self.radius * self.radius * std::f32::consts::PI
        }
    }
}

fn main() {
    // example1_io();
    // example2_io();

    // example3_std_types();
    // example4_control_flow();

    // example5_arrays_and_loops();
    // example6_tuples();

    // example7_strings();
    // example8_casting();
    // example9_enums();
    // example10_vectors();

    // example11_functions();
    // example12_generics();

    // example13_ownership();

    example14_hashmaps();
    example15_structs();
}
