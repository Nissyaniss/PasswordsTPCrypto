use std::{
	fs::{self, File},
	path::Path,
	process::exit,
};

use inquire::{validator::Validation, CustomType, Select, Text};
use sha2::{Digest, Sha256};

use crate::{string_ext::StringExt, terminal_utils};

pub fn main() {
	let is_windows = cfg!(windows);
	let path = if is_windows {
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
		change_password(&path, true);
		terminal_utils::clear_terminal();
	}

	loop {
		let options = vec![
			"Changer de mot de passe maitre",
			"Générer un mot de passe",
			"Quitté",
		];

		let ans = Select::new("Options :", options).prompt();

		match ans {
			Ok("Changer de mot de passe maitre") => {
				change_password(&path, false);
			}
			Ok("Générer un mot de passe") => {
				generate_password(&path);
			}
			Ok("Quitté") => {
				terminal_utils::clear_terminal();
				return;
			}
			Ok(choice) => println!("{choice} PAS FINIT/IMPLÉMENTÉ!"),
			Err(_) => println!("Il y a eu une erreur"),
		}
	}
}

fn generate_password(path: &str) {
	let password = fs::read_to_string(path).expect("Impossible de lire le fichier");
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

	let amount = CustomType::<u32>::new("La taille de votre mot de passe généré ?")
		.with_error_message("Une valeur valide doit être entre 1 et 12")
		.with_validator(|val: &u32| {
			if *val > 12u32 {
				Ok(Validation::Invalid(
					"Une valeur valide doit être entre 1 et 12".into(),
				))
			} else {
				Ok(Validation::Valid)
			}
		})
		.prompt()
		.unwrap();

	let password_final = password + &tag.unwrap();
	terminal_utils::clear_terminal();
	let mut hasher = Sha256::new();
	hasher.update(password_final);

	print!("Les {amount} premiers caractère de votre mot de passe sont : ");
	println!("{}", &hex::encode(hasher.finalize())[..amount as usize]);
}

pub fn change_password(path: &str, is_first_password: bool) {
	if is_first_password {
		File::create(path).unwrap();
		let password = Text::new("Quel est votre mot de passe maitre ?")
			.with_validator(|val: &str| {
				if val.contains(char::is_whitespace) {
					Ok(Validation::Invalid("Sans espaces".into()))
				} else {
					Ok(Validation::Valid)
				}
			})
			.prompt();
		if password.is_ok() {
			fs::write(path, password.unwrap()).expect("Impossible d'écrire le fichier");
		}
	} else {
		let ancient_password = fs::read_to_string(path).expect("Impossible d'écrire le fichier");
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
				println!("Trop d'essais");
				exit(1);
			}
			tries += 1;
		}
		let new_password = Text::new("Quel est votre nouveaux mot de passe maitre ?")
			.with_validator(|val: &str| {
				if val.contains(char::is_whitespace) {
					Ok(Validation::Invalid("Sans espaces".into()))
				} else {
					Ok(Validation::Valid)
				}
			})
			.prompt();
		if new_password.is_ok() {
			fs::write(path, new_password.unwrap()).expect("Impossible d'écrire le fichier");
		}
	}
	terminal_utils::clear_terminal();
	println!("Password sauvegardé");
}
