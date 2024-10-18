use crate::string_ext;

pub trait CharExt {
	fn get_ascii_printable_position(&self) -> Option<usize>;
}

impl CharExt for char {
	fn get_ascii_printable_position(&self) -> Option<usize> {
		for (j, printable) in string_ext::ASCIITABLE.chars().enumerate() {
			if *self == printable {
				return Some(j);
			}
		}
		None
	}
}
