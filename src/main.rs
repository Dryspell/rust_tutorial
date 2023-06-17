#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn example1() {
  println!("What's your name?");

  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Failed to read line");

  let greeting = "Nice to meet you!";

  println!("Hello, {}! {}", name.trim(), greeting);

}

fn example2() {
  const ONE_MILLION: u32 = 1_000_000;
  const PI: f32 = 3.14159;
  let age= "47";
  let mut age:u32 = age.trim().parse().expect("Please type a number!");
  age = age + 1;
  print!("age: {}, and I want ${}", age, ONE_MILLION);
}

fn example3_std_types(){
  println!("Max u32: {}", std::u32::MAX);
  println!("Max u64: {}", std::u64::MAX);
  println!("Max u128: {}", std::u128::MAX);
  println!("Max usize: {}", std::usize::MAX);


  println!("Max i32: {}", std::i32::MAX);
  println!("Max f32: {}", std::f32::MAX);
  println!("Max f64: {}", std::f64::MAX);

  let mut rng = rand::thread_rng();

}

fn main() {
  // example1();
  // example2();

  example3_std_types();
}
