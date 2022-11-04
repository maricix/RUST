use std::io;

fn main() {
    println!("Fibonacci series. Type 'q' for quitting.\nInput number:");
	
	loop {
		let mut input = String::new();
		io::stdin()
				.read_line(&mut input)
				.expect("Failed to read input");
		if input.trim() == "q" {
				break;
		}
		let number: u32 =  match input.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("Please input a number! {input}");
					continue 
				},
		};
		println!("Fibonacci({number}) = {}",fibonacci(number));
	}
}

fn fibonacci(n: u32)-> u32 {
	if (n == 0 || n==1) {
		return n;
	} else {
		fibonacci(n-1) + fibonacci(n-2)	
	}	
}	
