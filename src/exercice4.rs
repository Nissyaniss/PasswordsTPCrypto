use std::{fs, path::Path, process::exit};

use sha2::{Digest, Sha256};

use crate::{
	char_ext::CharExt,
	exercice3,
	string_ext::{self, ASCIITABLE},
	terminal_utils,
};

pub fn main() {
	let path = if cfg!(windows) {
		match option_env!("USERPROFILE") {
			Some(_) => std::env::var("USERPROFILE").unwrap() + "\\.mpwd.txt",
			None => exit(1),
		}
	} else {
		match option_env!("HOME") {
			Some(_) => std::env::var("HOME").unwrap() + "/.mpwd.txt",
			None => exit(1),
		}
	};

	if !Path::new(&path).exists() {
		exercice3::change_password(&path, true);
		terminal_utils::clear_terminal();
	}
	let password = fs::read_to_string(path).unwrap();

	let tag = "Unilim";

	let password_final = password + &tag;
	let mut hasher = Sha256::new();
	hasher.update(password_final);
	let hashed_password = hasher.finalize();

	let mut test = "aa~".to_string();
	for i in 0..5 {
		println!("{} : {}", i, test);
		test = update_password(test);
	}
	println!("{}", test);
	/*	loop {

		}
	*/ /*	loop {
		 tag = Text::new("Quelle est votre tag ?").prompt();
		 if tag.is_ok()
			 && tag.as_ref().unwrap().is_ascii_printable()
			 && !tag.as_ref().unwrap().is_empty()
		 {
			 break;
		 }
	 }*/
}

fn update_password(mut password: String) -> String {
	for i in (0..password.len()).rev() {
		let last_ascii_char = string_ext::ASCIITABLE.chars().nth(93).unwrap();
		let first_ascii_char = string_ext::ASCIITABLE.chars().next().unwrap();
		let current_char = password.chars().nth(i).unwrap();
		println!("{} - 1 < {}", i, password.len() - 1);
		if current_char == last_ascii_char {
			password.replace_range(
				i..=i,
				ASCIITABLE.chars().next().unwrap().to_string().as_str(),
			);
		} else if i - 1 < password.len() - 1
			&& password.chars().nth(i - 1).unwrap() != first_ascii_char
		{
			password.replace_range(
				i..=i,
				ASCIITABLE
					.chars()
					.nth(current_char.get_ascii_printable_position().unwrap() + 1)
					.unwrap()
					.to_string()
					.as_str(),
			);
		}
	}
	password
}
