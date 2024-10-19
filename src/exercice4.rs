use inquire::{validator::Validation, Confirm, CustomType, Text};
use itertools::{iproduct, Itertools};
use sha2::{Digest, Sha256};

use crate::{string_ext, terminal_utils};

pub fn main() {
	let password = Text::new("Quel est le mot de passe choisi (10 characters min et max) ?")
		.with_validator(|val: &str| {
			if val.len() == 10 {
				Ok(Validation::Valid)
			} else {
				Ok(Validation::Invalid(
					"Un mot de passe doit faire 10 caractere !".into(),
				))
			}
		})
		.prompt()
		.unwrap();
	let amount = CustomType::<usize>::new("La taille de votre mot de passe généré ?")
		.with_validator(|val: &usize| {
			if *val > 3usize || *val < 1usize {
				Ok(Validation::Invalid(
					"Une valeur valide doit être entre 1 et 3".into(),
				))
			} else {
				Ok(Validation::Valid)
			}
		})
		.prompt()
		.unwrap();
	let is_three_collision = Confirm::new("Est ce que vous voulez faire 3 collisions?")
		.with_default(false)
		.prompt()
		.unwrap();

	find_collisions(password, amount, is_three_collision);
}

fn find_collisions(password: String, amount: usize, is_three_collision: bool) {
	let hashed_password_unilim = hash_password(password.clone(), "Unilim", amount);
	let hashed_password_amazon = hash_password(password.clone(), "Amazon", amount);
	let hashed_password_netflix = hash_password(password, "Netflix", amount);

	let mut tries: u32 = 0;

	for (password, i) in iproduct!(
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied(),
		string_ext::ASCIITABLE.iter().copied()
	)
	.zip(0..)
	{
		let password = [
			password.0, password.1, password.2, password.3, password.4, password.5, password.6,
			password.7, password.8, password.8, password.9,
		]
		.iter()
		.join("");

		if is_three_collision {
			if hash_password(password.clone(), "Unilim", amount) == hashed_password_unilim
				&& hash_password(password.clone(), "Netflix", amount) == hashed_password_netflix
				&& hash_password(password.clone(), "Amazon", amount) == hashed_password_amazon
			{
				println!("Collisions trouvé sur {password} au bout de {i} essais");
			}
		} else if hash_password(password.clone(), "Unilim", amount) == hashed_password_unilim {
			terminal_utils::clear_terminal();
			println!("Collisions trouvé sur {password} au bout de {i} essais");
			return;
		}

		tries += 1;
		if tries % 1_000_000 == 0 {
			println!("Essais : {tries}");
		}
	}
}

fn hash_password(password: String, tag: &str, size: usize) -> Vec<u8> {
	let password_final = password + tag;
	let mut hasher = Sha256::new();
	hasher.update(password_final);
	hasher.finalize()[0..size].to_vec()
}
