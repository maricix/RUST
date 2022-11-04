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
		let number: usize =  match input.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("Please input a number! {input}");
					continue 
				},
		};
		let mut a = vec![0; number + 1];
		fibonacci(number, &mut a);
		print!("Fibonacci({number}) = ");
		for element in a {
			print!(" {element}");
		}
		println!();
	}
}

fn fibonacci(n: usize, arr: &mut [usize]) {
	if n == 0 || n==1 {
		arr[n] = n;
	} else {
		fibonacci(n-1, arr);
		fibonacci(n-2, arr);
		arr[n] = arr[n-1] + arr[n-2];	
	}	
}	
