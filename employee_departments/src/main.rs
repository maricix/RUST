use std::io;
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let add_pattern = Regex::new(r"^Add ([\w\s]+) to ([\w\s]+)").unwrap();

    let mut employees :HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("\nChoose action:\n 1. Add employer\n 2. Get list of employers by department \n \
        3. Get list of employers in a department\n 0. Quit");

        let choice: u32 = match read_input().parse() {
            Ok(num) => {
                if num > 3 {
                    println!("You must choose between '0' and '3'!");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("You must choose between '0' and '3'!");
                continue;
            },
        };

        match choice {
            1 => {
                println!("Add an employer like this: 'Add Sally to Engineering'. \nType 0 to go back to the menu.");
                let add_statement = read_input();
                let captures = add_pattern.captures(&add_statement);
                if captures.is_none() {
                    continue;
                }
                let captures = captures.unwrap();
                add_employee(&mut employees, captures.get(1).unwrap().as_str(),
                             captures.get(2).unwrap().as_str());
            },
            2 => {
                println!("Choose department:");
                let dept = read_input();
                show_department(&mut employees, &*dept);
            },
            3 => show_all_employees(&mut employees),
            _ => break
        }
    }
}

fn add_employee(employees :&mut HashMap<String, Vec<String>>, name: &str, dept: &str) {
    employees.entry(dept.parse().unwrap()).or_insert(Vec::new()).push(name.parse().unwrap());
    println!("{} added as employee to the {} department", name, dept);
}

fn show_department(employees :&mut HashMap<String, Vec<String>>, dept: &str) {
    match employees.get(dept) {
        Some(list) => {
            let mut list2 = list.clone();
            list2.sort_by(|x,y| x.cmp(&y));
            println!("Employers in {} department: {:?}", dept, list2);
        },
        None => println!("No employers in {} department", dept)
    }
}

fn show_all_employees(employees :&mut HashMap<String, Vec<String>>) {
    let mut all_empl = employees.clone();
    for key in employees.keys().sorted() {
        let employees_in_dept = all_empl.get_mut(key).unwrap();
        employees_in_dept.sort_by(|x,y| x.cmp(&y));
        println!("{}:{:?}", key, employees_in_dept);
    }
}

fn read_input()-> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}