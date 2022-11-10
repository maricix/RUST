use std::io;
use std::collections::HashMap;

fn main() {
    loop {
        let mut v: Vec<i32> = Vec::new();
        let mut num_freq_map:HashMap<i32,usize> = HashMap::new();
        for number in get_input().split_whitespace() {
            let i = number.parse().unwrap();
            v.push(i);
            *num_freq_map.entry(i).or_insert(0) +=1;
        }
        v.sort();
        println!("median = {}", median(&v));
        println!("mean = {}", mean(&v));
        println!("mode = {}", mode(num_freq_map));
        break;
    }
}

fn get_input() -> String {
    println!("Please input a list of signed integers ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read list of integers");
    let str = input.trim();
    if str.len() == 0 {
        panic!("Please input at least one number");
    }
    str.to_string()
}

fn median(v: &Vec<i32>) -> f64 {
    let length = v.len();
    let middle = length / 2;
    if length % 2 == 0 {
        (v[middle - 1] + v[middle]) as f64 / 2.0
    } else {
        v[middle] as f64
    }
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut mean: f64 = 0.0;
    for number in v {
        mean += *number as f64;
    }
    mean / v.len() as f64
}

fn mode(map: HashMap<i32,usize>) -> i32 {
    let mut mode = 0;
    let mut freq :usize = 0;
    for (key,value) in map {
        if value > freq {
            freq = value;
            mode = key;
        }
    }
    if freq == 1 {
        panic!("All values appear just once!")
    }
    mode
}