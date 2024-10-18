use std::{
	fs::{self, File},
	path::Path,
	process::exit,
};

use inquire::{validator::Validation, CustomType, Select, Text};
use sha2::{Digest, Sha256};

use crate::{string_ext::StringExt, terminal_utils};

pub fn main() {
	loop {
		let options = vec![
			"Changer de mot de passe maitre",
			"Générer un mot de passe",
			"Quit",
		];

		let ans = Select::new("Options :", options).prompt();

		match ans {
			Ok("Changer de mot de passe maitre") => {
				if cfg!(windows) {
					let home = match option_env!("USERPROFILE") {
						Some(_) => option_env!("USERPROFILE"),
						None => exit(1),
					};
					if Path::new(&format!("{}\\.mpwd.txt", home.unwrap())).exists() {
						change_password(&format!("{}\\mdpw.txt", home.unwrap()), true);
					} else {
						change_password(&format!("{}\\mdpw.txt", home.unwrap()), false);
					}
				} else {
					let home = match option_env!("HOME") {
						Some(_) => option_env!("HOME"),
						None => exit(1),
					};
					if Path::new(&format!("{}/.mpwd.txt", home.unwrap())).exists() {
						change_password(&format!("{}/mdpw.txt", home.unwrap()), true);
					} else {
						change_password(&format!("{}/mdpw.txt", home.unwrap()), false);
					}
				}
			}
			Ok("Générer un mot de passe") => {
				if cfg!(windows) {
					let home = match option_env!("USERPROFILE") {
						Some(_) => option_env!("USERPROFILE"),
						None => exit(1),
					};
					generate_password(&format!("{}\\mdpw.txt", home.unwrap()));
				} else {
					let home = match option_env!("HOME") {
						Some(_) => option_env!("HOME"),
						None => exit(1),
					};
					generate_password(&format!("{}/mdpw.txt", home.unwrap()));
				}
			}
			Ok("Quit") => {
				terminal_utils::clear_terminal();
				return;
			}
			Ok(choice) => println!("{choice} NOT FINIHED!"),
			Err(_) => println!("There was an error, please try again"),
		}
	}
}

fn generate_password(path: &str) {
	let password = fs::read_to_string(path).expect("Unable to read file");
	let mut tag;

	loop {
		tag = Text::new("Quelle est votre tag ?").prompt();
		if tag.is_ok()
			&& tag.as_ref().unwrap().is_ascii_printable()
			&& !tag.as_ref().unwrap().is_empty()
		{
			break;
		}
	}

	let amount = CustomType::<u32>::new("The size of the output wanted ?")
		.with_error_message("Please type a valid number between 1 and 12")
		.with_validator(|val: &u32| {
			if *val > 12u32 {
				Ok(Validation::Invalid(
					"Please type a valid number between 1 and 12".into(),
				))
			} else {
				Ok(Validation::Valid)
			}
		})
		.prompt()
		.unwrap();

	let string3 = password + &tag.unwrap();
	terminal_utils::clear_terminal();
	let mut hasher = Sha256::new();
	hasher.update(string3);

	print!("The {amount} first characters of the hash is : ");
	println!("{}", &hex::encode(hasher.finalize())[..amount as usize]);
}

fn change_password(path: &str, is_first_password: bool) {
	if is_first_password {
		File::create(path).unwrap();
		let password = Text::new("Quel est votre mot de passe maitre ?")
			.with_validator(|val: &str| {
				if val.contains(char::is_whitespace) {
					Ok(Validation::Invalid("Sans espaces stp".into()))
				} else {
					Ok(Validation::Valid)
				}
			})
			.prompt();
		if password.is_ok() {
			fs::write(path, password.unwrap()).expect("Unable to write file");
		}
	} else {
		let ancient_password = fs::read_to_string(path).expect("Unable to read file");
		let mut tries: u32 = 0;
		loop {
			let password_check = Text::new("Quel est votre mot de passe maitre actuel ?")
				.with_validator(|val: &str| {
					if val.contains(char::is_whitespace) {
						Ok(Validation::Invalid("Sans espaces stp".into()))
					} else {
						Ok(Validation::Valid)
					}
				})
				.prompt();
			if password_check.is_ok() && *password_check.as_ref().unwrap() == ancient_password {
				break;
			}
			if tries == 2 {
				println!("Too many tries");
				exit(1);
			}
			tries += 1;
		}
		let new_password = Text::new("Quel est votre nouveaux mot de passe maitre ?")
			.with_validator(|val: &str| {
				if val.contains(char::is_whitespace) {
					Ok(Validation::Invalid("Sans espaces stp".into()))
				} else {
					Ok(Validation::Valid)
				}
			})
			.prompt();
		if new_password.is_ok() {
			fs::write(path, new_password.unwrap()).expect("Unable to write file");
		}
	}
	println!("Password saved");
}
