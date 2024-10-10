use crate::string_ext::StringExt;
use crate::terminal_utils;
use inquire::Text;
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

	let string3 = string1.unwrap() + &string2.unwrap();
	terminal_utils::clear_terminal();
	let mut hasher = Sha256::new();
	hasher.update(string3);
	let hash = hasher.finalize();
	print!("The 8 first characters of the hash is : ");
	for (i, b) in hash.into_iter().enumerate() {
		if i == 4 {
			break;
		}
		print!("{b:x}");
		terminal_utils::flush();
	}
	println!();
}
