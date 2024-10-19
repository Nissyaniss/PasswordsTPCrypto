use inquire::Select;
use std::process::exit;

mod exercice1;
mod exercice2;
mod exercice3;
mod exercice4;
mod string_ext;
mod terminal_utils;

fn main() {
	terminal_utils::clear_terminal();

	loop {
		let options = vec![
			"Exercice 1",
			"Exercice 2",
			"Exercice 3",
			"Exercice 4",
			"Quitter",
		];

		let ans = Select::new("Exercice que tu veux", options).prompt();

		match ans {
			Ok("Exercice 1") => exercice1::main(),
			Ok("Exercice 2") => exercice2::main(),
			Ok("Exercice 3") => exercice3::main(),
			Ok("Exercice 4") => exercice4::main(),
			Ok("Quitter") => exit(0),
			Ok(choice) => println!("{choice} NOT FINIHED!"),
			Err(_) => println!("There was an error, please try again"),
		}
	}
}
