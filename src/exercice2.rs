use crate::string_ext::StringExt;
use crate::terminal_utils;
use inquire::validator::Validation;
use inquire::{CustomType, Text};
use sha2::{Digest, Sha256};

pub fn main() {
	let mut string1;
	let mut string2;
	loop {
		string1 = Text::new("What is your first string ?").prompt();
		if string1.is_ok()
			&& string1.as_ref().unwrap().is_ascii_printable()
			&& !string1.as_ref().unwrap().is_empty()
		{
			break;
		}
	}

	loop {
		string2 = Text::new("What is your second string ?").prompt();
		if string2.is_ok()
			&& string2.as_ref().unwrap().is_ascii_printable()
			&& !string2.as_ref().unwrap().is_empty()
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

	let string3 = string1.unwrap() + &string2.unwrap();
	terminal_utils::clear_terminal();
	let mut hasher = Sha256::new();
	hasher.update(string3);

	print!("The {amount} first characters of the hash is : ");
	println!("{}", &hex::encode(hasher.finalize())[..amount as usize]);
}
