use std::io;

fn main() {
	'convertion_choice: loop {
		println!("Convert temperatures!\n Type '1' for converting from Celsius to Fahrenheit.\n Type '2' for converting from Fahrenheit to Celsius.\n Type 'q' for quitting.");
		let choice = read_input();
		if choice.trim() == "q" {
			break;
		}
		let choice: u32 =  match choice.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("You must choose between '1' and '2'!");
					continue 
				},
		};	

		match choice {
			1 => println!("Input temperature in Celsius. Type 'q' to quit. Type 'c' to change the conversion type."),
			2 => println!("Input temperature in Fahrenheit. Type 'q' to quit. Type 'c' to change the conversion type."),
			_ => {
				println!("You must choose between '1' and '2'. Type 'q' to quit. Type 'c' to change the conversion type.");
				continue
			},
		}
		
		loop {
			let temperature = read_input();
			match temperature.trim() {
				"q" => break 'convertion_choice,
				"c" => break,
				_ => {
					let temperature: f64 =  match temperature.trim().parse() {
						Ok(num) => num,
						Err(_) => {
							println!("Input float number, please");
							continue
						},
					};
					match choice {
						1 => println!("{temperature}C = {}F", chelsius_to_fahrenheit(temperature)),
						2 => println!("{temperature}F = {}C", fahrenheit_to_chelsius(temperature)),
						_ => {
							println!("Input float number, please. Type 'q' to quit. Type 'c' to change the conversion type.");
							continue
						},	
					};				
				},
			}			
		}
	}
}

fn read_input()-> String {
	let mut input = String::new();
	io::stdin()
			.read_line(&mut input)
			.expect("Failed to read input");
	input
}

fn chelsius_to_fahrenheit(x:f64) -> f64 {
	(x * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_chelsius(x:f64) -> f64 {
	(x - 32.0) * 5.0 / 9.0
}